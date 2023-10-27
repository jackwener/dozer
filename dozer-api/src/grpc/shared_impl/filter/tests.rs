use dozer_cache::cache::test_utils::schema_1;
use dozer_types::serde_json::json;

use super::*;

fn test_field_satisfies_op_impl(
    field: value::Value,
    operator: Operator,
    value: Field,
    expected: bool,
) {
    assert_eq!(
        field_satisfies_op(&Value { value: Some(field) }, operator, &value),
        expected
    );
}

#[test]
fn test_field_satisfies_op() {
    test_field_satisfies_op_impl(
        value::Value::UintValue(1),
        Operator::LT,
        Field::UInt(2),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::UintValue(1),
        Operator::LT,
        Field::UInt(1),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::UintValue(1),
        Operator::LT,
        Field::UInt(0),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::UintValue(1),
        Operator::LTE,
        Field::UInt(2),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::UintValue(1),
        Operator::LTE,
        Field::UInt(1),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::UintValue(1),
        Operator::LTE,
        Field::UInt(0),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::UintValue(1),
        Operator::EQ,
        Field::UInt(2),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::UintValue(1),
        Operator::EQ,
        Field::UInt(1),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::UintValue(1),
        Operator::EQ,
        Field::UInt(0),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::UintValue(1),
        Operator::GT,
        Field::UInt(2),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::UintValue(1),
        Operator::GT,
        Field::UInt(1),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::UintValue(1),
        Operator::GT,
        Field::UInt(0),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::UintValue(1),
        Operator::GTE,
        Field::UInt(2),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::UintValue(1),
        Operator::GTE,
        Field::UInt(1),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::UintValue(1),
        Operator::GTE,
        Field::UInt(0),
        true,
    );

    test_field_satisfies_op_impl(value::Value::IntValue(1), Operator::LT, Field::Int(2), true);
    test_field_satisfies_op_impl(
        value::Value::IntValue(1),
        Operator::LT,
        Field::Int(1),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::IntValue(1),
        Operator::LT,
        Field::Int(0),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::IntValue(1),
        Operator::LTE,
        Field::Int(2),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::IntValue(1),
        Operator::LTE,
        Field::Int(1),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::IntValue(1),
        Operator::LTE,
        Field::Int(0),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::IntValue(1),
        Operator::EQ,
        Field::Int(2),
        false,
    );
    test_field_satisfies_op_impl(value::Value::IntValue(1), Operator::EQ, Field::Int(1), true);
    test_field_satisfies_op_impl(
        value::Value::IntValue(1),
        Operator::EQ,
        Field::Int(0),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::IntValue(1),
        Operator::GT,
        Field::Int(2),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::IntValue(1),
        Operator::GT,
        Field::Int(1),
        false,
    );
    test_field_satisfies_op_impl(value::Value::IntValue(1), Operator::GT, Field::Int(0), true);
    test_field_satisfies_op_impl(
        value::Value::IntValue(1),
        Operator::GTE,
        Field::Int(2),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::IntValue(1),
        Operator::GTE,
        Field::Int(1),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::IntValue(1),
        Operator::GTE,
        Field::Int(0),
        true,
    );

    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::LT,
        Field::Float(OrderedFloat(2.0)),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::LT,
        Field::Float(OrderedFloat(1.0)),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::LT,
        Field::Float(OrderedFloat(0.0)),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::LTE,
        Field::Float(OrderedFloat(2.0)),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::LTE,
        Field::Float(OrderedFloat(1.0)),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::LTE,
        Field::Float(OrderedFloat(0.0)),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::EQ,
        Field::Float(OrderedFloat(2.0)),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::EQ,
        Field::Float(OrderedFloat(1.0)),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::EQ,
        Field::Float(OrderedFloat(0.0)),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::GT,
        Field::Float(OrderedFloat(2.0)),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::GT,
        Field::Float(OrderedFloat(1.0)),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::GT,
        Field::Float(OrderedFloat(0.0)),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::GTE,
        Field::Float(OrderedFloat(2.0)),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::GTE,
        Field::Float(OrderedFloat(1.0)),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::GTE,
        Field::Float(OrderedFloat(0.0)),
        true,
    );

    test_field_satisfies_op_impl(
        value::Value::BoolValue(true),
        Operator::LT,
        Field::Boolean(true),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::BoolValue(true),
        Operator::LT,
        Field::Boolean(false),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::BoolValue(false),
        Operator::LT,
        Field::Boolean(true),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::BoolValue(false),
        Operator::LT,
        Field::Boolean(false),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::BoolValue(true),
        Operator::LTE,
        Field::Boolean(true),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::BoolValue(true),
        Operator::LTE,
        Field::Boolean(false),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::BoolValue(false),
        Operator::LTE,
        Field::Boolean(true),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::BoolValue(false),
        Operator::LTE,
        Field::Boolean(false),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::BoolValue(true),
        Operator::EQ,
        Field::Boolean(true),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::BoolValue(true),
        Operator::EQ,
        Field::Boolean(false),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::BoolValue(false),
        Operator::EQ,
        Field::Boolean(true),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::BoolValue(false),
        Operator::EQ,
        Field::Boolean(false),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::BoolValue(true),
        Operator::GT,
        Field::Boolean(true),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::BoolValue(true),
        Operator::GT,
        Field::Boolean(false),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::BoolValue(false),
        Operator::GT,
        Field::Boolean(true),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::BoolValue(false),
        Operator::GT,
        Field::Boolean(false),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::BoolValue(true),
        Operator::GTE,
        Field::Boolean(true),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::BoolValue(true),
        Operator::GTE,
        Field::Boolean(false),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::BoolValue(false),
        Operator::GTE,
        Field::Boolean(true),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::BoolValue(false),
        Operator::GTE,
        Field::Boolean(false),
        true,
    );

    test_field_satisfies_op_impl(
        value::Value::StringValue("b".into()),
        Operator::LT,
        Field::String("c".into()),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::StringValue("b".into()),
        Operator::LT,
        Field::String("b".into()),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::StringValue("b".into()),
        Operator::LT,
        Field::String("a".into()),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::StringValue("b".into()),
        Operator::LTE,
        Field::String("c".into()),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::StringValue("b".into()),
        Operator::LTE,
        Field::String("b".into()),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::StringValue("b".into()),
        Operator::LTE,
        Field::String("a".into()),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::StringValue("b".into()),
        Operator::EQ,
        Field::String("c".into()),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::StringValue("b".into()),
        Operator::EQ,
        Field::String("b".into()),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::StringValue("b".into()),
        Operator::EQ,
        Field::String("a".into()),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::StringValue("b".into()),
        Operator::GT,
        Field::String("c".into()),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::StringValue("b".into()),
        Operator::GT,
        Field::String("b".into()),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::StringValue("b".into()),
        Operator::GT,
        Field::String("a".into()),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::StringValue("b".into()),
        Operator::GTE,
        Field::String("c".into()),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::StringValue("b".into()),
        Operator::GTE,
        Field::String("b".into()),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::StringValue("b".into()),
        Operator::GTE,
        Field::String("a".into()),
        true,
    );

    test_field_satisfies_op_impl(
        value::Value::BytesValue(vec![1]),
        Operator::LT,
        Field::Binary(vec![2]),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::BytesValue(vec![1]),
        Operator::LT,
        Field::Binary(vec![1]),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::BytesValue(vec![1]),
        Operator::LT,
        Field::Binary(vec![0]),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::BytesValue(vec![1]),
        Operator::LTE,
        Field::Binary(vec![2]),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::BytesValue(vec![1]),
        Operator::LTE,
        Field::Binary(vec![1]),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::BytesValue(vec![1]),
        Operator::LTE,
        Field::Binary(vec![0]),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::BytesValue(vec![1]),
        Operator::EQ,
        Field::Binary(vec![2]),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::BytesValue(vec![1]),
        Operator::EQ,
        Field::Binary(vec![1]),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::BytesValue(vec![1]),
        Operator::EQ,
        Field::Binary(vec![0]),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::BytesValue(vec![1]),
        Operator::GT,
        Field::Binary(vec![2]),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::BytesValue(vec![1]),
        Operator::GT,
        Field::Binary(vec![1]),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::BytesValue(vec![1]),
        Operator::GT,
        Field::Binary(vec![0]),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::BytesValue(vec![1]),
        Operator::GTE,
        Field::Binary(vec![2]),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::BytesValue(vec![1]),
        Operator::GTE,
        Field::Binary(vec![1]),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::BytesValue(vec![1]),
        Operator::GTE,
        Field::Binary(vec![0]),
        true,
    );

    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::LT,
        Field::Float(OrderedFloat(2.0)),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::LT,
        Field::Float(OrderedFloat(1.0)),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::LT,
        Field::Float(OrderedFloat(0.0)),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::LTE,
        Field::Float(OrderedFloat(2.0)),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::LTE,
        Field::Float(OrderedFloat(1.0)),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::LTE,
        Field::Float(OrderedFloat(0.0)),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::EQ,
        Field::Float(OrderedFloat(2.0)),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::EQ,
        Field::Float(OrderedFloat(1.0)),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::EQ,
        Field::Float(OrderedFloat(0.0)),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::GT,
        Field::Float(OrderedFloat(2.0)),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::GT,
        Field::Float(OrderedFloat(1.0)),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::GT,
        Field::Float(OrderedFloat(0.0)),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::GTE,
        Field::Float(OrderedFloat(2.0)),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::GTE,
        Field::Float(OrderedFloat(1.0)),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::FloatValue(1.0),
        Operator::GTE,
        Field::Float(OrderedFloat(0.0)),
        true,
    );

    test_field_satisfies_op_impl(
        value::Value::StringValue("abc".into()),
        Operator::Contains,
        Field::String("abc".into()),
        true,
    );
    test_field_satisfies_op_impl(
        value::Value::StringValue("abc".into()),
        Operator::Contains,
        Field::String("d".into()),
        false,
    );

    test_field_satisfies_op_impl(
        value::Value::UintValue(0),
        Operator::LT,
        Field::Int(0),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::UintValue(0),
        Operator::LTE,
        Field::Int(0),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::UintValue(0),
        Operator::EQ,
        Field::Int(0),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::UintValue(0),
        Operator::GT,
        Field::Int(0),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::UintValue(0),
        Operator::GTE,
        Field::Int(0),
        false,
    );
    test_field_satisfies_op_impl(
        value::Value::UintValue(0),
        Operator::Contains,
        Field::Int(0),
        false,
    );
}

#[test]
fn test_record_satisfies_filter() {
    let schema = schema_1().0;
    let record = Record {
        values: vec![
            Value {
                value: Some(value::Value::IntValue(1)),
            },
            Value {
                value: Some(value::Value::StringValue("b".into())),
            },
            Value {
                value: Some(value::Value::IntValue(3)),
            },
        ],
        id: 1,
        version: 1,
    };

    let check = |filter, expected| {
        assert_eq!(record_satisfies_filter(&record, &filter, &schema), expected);
    };

    check(
        FilterExpression::Simple("a".into(), Operator::EQ, json!(1)),
        true,
    );
    check(
        FilterExpression::Simple("a".into(), Operator::EQ, json!(2)),
        false,
    );
    check(
        FilterExpression::And(vec![
            FilterExpression::Simple("a".into(), Operator::EQ, json!(1)),
            FilterExpression::Simple("b".into(), Operator::EQ, "b".into()),
        ]),
        true,
    );
    check(
        FilterExpression::And(vec![
            FilterExpression::Simple("a".into(), Operator::EQ, json!(1)),
            FilterExpression::Simple("b".into(), Operator::EQ, "c".into()),
        ]),
        false,
    );
}

#[test]
fn test_op_satisfies_filter() {
    let schema = schema_1().0;
    let old = Record {
        values: vec![
            Value {
                value: Some(value::Value::IntValue(1)),
            },
            Value {
                value: Some(value::Value::StringValue("b".into())),
            },
            Value {
                value: Some(value::Value::IntValue(3)),
            },
        ],
        id: 1,
        version: 1,
    };
    let new = Record {
        values: vec![
            Value {
                value: Some(value::Value::IntValue(2)),
            },
            Value {
                value: Some(value::Value::StringValue("b".into())),
            },
            Value {
                value: Some(value::Value::IntValue(3)),
            },
        ],
        id: 1,
        version: 1,
    };
    let filter1 = FilterExpression::Simple("a".into(), Operator::EQ, json!(1));
    let filter2 = FilterExpression::Simple("a".into(), Operator::EQ, json!(2));
    let filter3 = FilterExpression::Simple("a".into(), Operator::EQ, json!(3));

    let check = |typ, old: Option<&Record>, new: &Record, filter, expected| {
        assert_eq!(
            op_satisfies_filter(
                &Operation {
                    typ: typ as _,
                    old: old.cloned(),
                    new: Some(new.clone()),
                    endpoint_name: "".into()
                },
                filter,
                &schema
            ),
            expected
        );
    };

    check(OperationType::Insert, None, &new, Some(&filter1), false);
    check(OperationType::Insert, None, &new, Some(&filter2), true);
    check(OperationType::Delete, None, &new, Some(&filter1), false);
    check(OperationType::Delete, None, &new, Some(&filter2), true);
    check(
        OperationType::Update,
        Some(&old),
        &new,
        Some(&filter1),
        true,
    );
    check(
        OperationType::Update,
        Some(&old),
        &new,
        Some(&filter2),
        true,
    );
    check(
        OperationType::Update,
        Some(&old),
        &new,
        Some(&filter3),
        false,
    );
}
