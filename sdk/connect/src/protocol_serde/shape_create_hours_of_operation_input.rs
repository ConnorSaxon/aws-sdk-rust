// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_hours_of_operation_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateHoursOfOperationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.config {
        let mut array_2 = object.key("Config").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_hours_of_operation_config::ser_hours_of_operation_config(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.description {
        object.key("Description").string(var_5.as_str());
    }
    if let Some(var_6) = &input.name {
        object.key("Name").string(var_6.as_str());
    }
    if let Some(var_7) = &input.tags {
        #[allow(unused_mut)]
        let mut object_8 = object.key("Tags").start_object();
        for (key_9, value_10) in var_7 {
             {
                object_8.key(key_9.as_str()).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    if let Some(var_11) = &input.time_zone {
        object.key("TimeZone").string(var_11.as_str());
    }
    Ok(())
}

