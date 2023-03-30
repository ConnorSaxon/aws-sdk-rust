// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_queue_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateQueueInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("Description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.hours_of_operation_id {
        object.key("HoursOfOperationId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.max_contacts {
        object.key("MaxContacts").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_3).into()));
    }
    if let Some(var_4) = &input.name {
        object.key("Name").string(var_4.as_str());
    }
    if let Some(var_5) = &input.outbound_caller_config {
        #[allow(unused_mut)]
        let mut object_6 = object.key("OutboundCallerConfig").start_object();
        crate::protocol_serde::shape_outbound_caller_config::ser_outbound_caller_config(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.quick_connect_ids {
        let mut array_8 = object.key("QuickConnectIds").start_array();
        for item_9 in var_7 {
             {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    if let Some(var_10) = &input.tags {
        #[allow(unused_mut)]
        let mut object_11 = object.key("Tags").start_object();
        for (key_12, value_13) in var_10 {
             {
                object_11.key(key_12.as_str()).string(value_13.as_str());
            }
        }
        object_11.finish();
    }
    Ok(())
}

