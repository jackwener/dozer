use dozer_ingestion_connector::{
    async_trait,
    dozer_types::{errors::internal::BoxedError, log::info, types::FieldType},
    utils::ListOrFilterColumns,
    Connector, Ingestor, SourceSchemaResult, TableIdentifier, TableInfo, TableToIngest,
};
use postgres_types::PgLsn;
use rand::distributions::Alphanumeric;
use rand::Rng;
use tokio_postgres::config::ReplicationMode;
use tokio_postgres::Config;

use crate::{
    connection::validator::validate_connection,
    iterator::PostgresIterator,
    schema::helper::{SchemaHelper, DEFAULT_SCHEMA_NAME},
    PostgresConnectorError,
};

use super::connection::client::Client;
use super::connection::helper;

#[derive(Clone, Debug)]
pub struct PostgresConfig {
    pub name: String,
    pub config: Config,
    pub schema: Option<String>,
}

#[derive(Debug)]
pub struct PostgresConnector {
    name: String,
    replication_conn_config: Config,
    conn_config: Config,
    schema_helper: SchemaHelper,
    pub schema: Option<String>,
}

#[derive(Debug)]
pub struct ReplicationSlotInfo {
    pub name: String,
    pub start_lsn: PgLsn,
}

pub const REPLICATION_SLOT_PREFIX: &str = "dozer_slot";

impl PostgresConnector {
    pub fn new(config: PostgresConfig) -> PostgresConnector {
        let mut replication_conn_config = config.config.clone();
        replication_conn_config.replication_mode(ReplicationMode::Logical);

        let helper = SchemaHelper::new(config.config.clone(), config.schema.clone());

        // conn_str - replication_conn_config
        // conn_str_plain- conn_config

        PostgresConnector {
            name: config.name,
            conn_config: config.config,
            replication_conn_config,
            schema_helper: helper,
            schema: config.schema,
        }
    }

    fn get_lsn_with_offset_from_seq(
        conn_name: String,
        from_seq: Option<(u64, u64)>,
    ) -> Option<(PgLsn, u64)> {
        from_seq.map_or_else(
            || {
                info!("[{}] Starting replication", conn_name);
                None
            },
            |(lsn, checkpoint)| {
                info!(
                    "[{}] Starting replication from checkpoint ({}/{})",
                    conn_name, lsn, checkpoint
                );
                Some((PgLsn::from(lsn), checkpoint))
            },
        )
    }
}

#[async_trait]
impl Connector for PostgresConnector {
    fn types_mapping() -> Vec<(String, Option<FieldType>)>
    where
        Self: Sized,
    {
        todo!()
    }

    async fn validate_connection(&self) -> Result<(), BoxedError> {
        validate_connection(&self.name, self.conn_config.clone(), None, None)
            .await
            .map_err(Into::into)
    }

    async fn list_tables(&self) -> Result<Vec<TableIdentifier>, BoxedError> {
        Ok(self
            .schema_helper
            .get_tables(None)
            .await?
            .into_iter()
            .map(|table| TableIdentifier::new(Some(table.schema), table.name))
            .collect())
    }

    async fn validate_tables(&self, tables: &[TableIdentifier]) -> Result<(), BoxedError> {
        let tables = tables
            .iter()
            .map(|table| ListOrFilterColumns {
                schema: table.schema.clone(),
                name: table.name.clone(),
                columns: None,
            })
            .collect::<Vec<_>>();
        validate_connection(&self.name, self.conn_config.clone(), Some(&tables), None)
            .await
            .map_err(Into::into)
    }

    async fn list_columns(
        &self,
        tables: Vec<TableIdentifier>,
    ) -> Result<Vec<TableInfo>, BoxedError> {
        let table_infos = tables
            .iter()
            .map(|table| ListOrFilterColumns {
                schema: table.schema.clone(),
                name: table.name.clone(),
                columns: None,
            })
            .collect::<Vec<_>>();
        Ok(self
            .schema_helper
            .get_tables(Some(&table_infos))
            .await?
            .into_iter()
            .map(|table| TableInfo {
                schema: Some(table.schema),
                name: table.name,
                column_names: table.columns,
            })
            .collect())
    }

    async fn get_schemas(
        &self,
        table_infos: &[TableInfo],
    ) -> Result<Vec<SourceSchemaResult>, BoxedError> {
        let table_infos = table_infos
            .iter()
            .map(|table| ListOrFilterColumns {
                schema: table.schema.clone(),
                name: table.name.clone(),
                columns: Some(table.column_names.clone()),
            })
            .collect::<Vec<_>>();
        Ok(self
            .schema_helper
            .get_schemas(&table_infos)
            .await?
            .into_iter()
            .map(|schema_result| schema_result.map_err(Into::into))
            .collect())
    }

    async fn start(
        &self,
        ingestor: &Ingestor,
        tables: Vec<TableToIngest>,
    ) -> Result<(), BoxedError> {
        let client = helper::connect(self.replication_conn_config.clone()).await?;
        let table_identifiers = tables
            .iter()
            .map(|table| TableIdentifier::new(table.schema.clone(), table.name.clone()))
            .collect::<Vec<_>>();
        self.create_publication(client, Some(&table_identifiers))
            .await?;

        let lsn = PostgresConnector::get_lsn_with_offset_from_seq(self.name.clone(), None);

        let tables = tables
            .into_iter()
            .map(|table| {
                assert!(table.checkpoint.is_none());
                ListOrFilterColumns {
                    schema: table.schema,
                    name: table.name,
                    columns: Some(table.column_names),
                }
            })
            .collect::<Vec<_>>();
        let iterator = PostgresIterator::new(
            self.name.clone(),
            self.get_publication_name(),
            self.get_slot_name(),
            self.schema_helper.get_tables(Some(&tables)).await?,
            self.replication_conn_config.clone(),
            ingestor,
            self.conn_config.clone(),
            self.schema.clone(),
        );
        iterator.start(lsn).await.map_err(Into::into)
    }
}

impl PostgresConnector {
    fn get_publication_name(&self) -> String {
        format!("dozer_publication_{}", self.name)
    }

    pub fn get_slot_name(&self) -> String {
        let rand_name_suffix: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        format!(
            "{REPLICATION_SLOT_PREFIX}_{}_{}",
            self.name,
            rand_name_suffix.to_lowercase()
        )
    }

    pub async fn create_publication(
        &self,
        mut client: Client,
        table_identifiers: Option<&[TableIdentifier]>,
    ) -> Result<(), PostgresConnectorError> {
        let publication_name = self.get_publication_name();
        let table_str: String = match table_identifiers {
            None => "ALL TABLES".to_string(),
            Some(table_identifiers) => {
                let table_names = table_identifiers
                    .iter()
                    .map(|table_identifier| {
                        format!(
                            r#""{}"."{}""#,
                            table_identifier
                                .schema
                                .as_deref()
                                .unwrap_or(DEFAULT_SCHEMA_NAME),
                            table_identifier.name
                        )
                    })
                    .collect::<Vec<_>>();
                format!("TABLE {}", table_names.join(" , "))
            }
        };

        client
            .simple_query(format!("DROP PUBLICATION IF EXISTS {publication_name}").as_str())
            .await
            .map_err(PostgresConnectorError::DropPublicationError)?;

        client
            .simple_query(format!("CREATE PUBLICATION {publication_name} FOR {table_str}").as_str())
            .await
            .map_err(PostgresConnectorError::CreatePublicationError)?;

        Ok(())
    }
}
