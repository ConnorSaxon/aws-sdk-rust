// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_task(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Task) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.connector_operator {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ConnectorOperator").start_object();
        crate::protocol_serde::shape_connector_operator::ser_connector_operator(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.destination_field {
        object.key("DestinationField").string(var_3.as_str());
    }
    if let Some(var_4) = &input.source_fields {
        let mut array_5 = object.key("SourceFields").start_array();
        for item_6 in var_4 {
             {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.task_properties {
        #[allow(unused_mut)]
        let mut object_8 = object.key("TaskProperties").start_object();
        for (key_9, value_10) in var_7 {
             {
                object_8.key(key_9.as_str()).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    if let Some(var_11) = &input.task_type {
        object.key("TaskType").string(var_11.as_str());
    }
    Ok(())
}

