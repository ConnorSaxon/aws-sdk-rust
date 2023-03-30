// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_scan_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ScanInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.table_name {
        object.key("TableName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.index_name {
        object.key("IndexName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.attributes_to_get {
        let mut array_4 = object.key("AttributesToGet").start_array();
        for item_5 in var_3 {
             {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.limit {
        object.key("Limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_6).into()));
    }
    if let Some(var_7) = &input.select {
        object.key("Select").string(var_7.as_str());
    }
    if let Some(var_8) = &input.scan_filter {
        #[allow(unused_mut)]
        let mut object_9 = object.key("ScanFilter").start_object();
        for (key_10, value_11) in var_8 {
             {
                #[allow(unused_mut)]
                let mut object_12 = object_9.key(key_10.as_str()).start_object();
                crate::protocol_serde::shape_condition::ser_condition(&mut object_12, value_11)?;
                object_12.finish();
            }
        }
        object_9.finish();
    }
    if let Some(var_13) = &input.conditional_operator {
        object.key("ConditionalOperator").string(var_13.as_str());
    }
    if let Some(var_14) = &input.exclusive_start_key {
        #[allow(unused_mut)]
        let mut object_15 = object.key("ExclusiveStartKey").start_object();
        for (key_16, value_17) in var_14 {
             {
                #[allow(unused_mut)]
                let mut object_18 = object_15.key(key_16.as_str()).start_object();
                crate::protocol_serde::shape_attribute_value::ser_attribute_value(&mut object_18, value_17)?;
                object_18.finish();
            }
        }
        object_15.finish();
    }
    if let Some(var_19) = &input.return_consumed_capacity {
        object.key("ReturnConsumedCapacity").string(var_19.as_str());
    }
    if let Some(var_20) = &input.total_segments {
        object.key("TotalSegments").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_20).into()));
    }
    if let Some(var_21) = &input.segment {
        object.key("Segment").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_21).into()));
    }
    if let Some(var_22) = &input.projection_expression {
        object.key("ProjectionExpression").string(var_22.as_str());
    }
    if let Some(var_23) = &input.filter_expression {
        object.key("FilterExpression").string(var_23.as_str());
    }
    if let Some(var_24) = &input.expression_attribute_names {
        #[allow(unused_mut)]
        let mut object_25 = object.key("ExpressionAttributeNames").start_object();
        for (key_26, value_27) in var_24 {
             {
                object_25.key(key_26.as_str()).string(value_27.as_str());
            }
        }
        object_25.finish();
    }
    if let Some(var_28) = &input.expression_attribute_values {
        #[allow(unused_mut)]
        let mut object_29 = object.key("ExpressionAttributeValues").start_object();
        for (key_30, value_31) in var_28 {
             {
                #[allow(unused_mut)]
                let mut object_32 = object_29.key(key_30.as_str()).start_object();
                crate::protocol_serde::shape_attribute_value::ser_attribute_value(&mut object_32, value_31)?;
                object_32.finish();
            }
        }
        object_29.finish();
    }
    if let Some(var_33) = &input.consistent_read {
        object.key("ConsistentRead").boolean(*var_33);
    }
    Ok(())
}

