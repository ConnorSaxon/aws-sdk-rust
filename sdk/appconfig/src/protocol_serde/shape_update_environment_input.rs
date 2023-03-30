// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_environment_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateEnvironmentInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("Description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.monitors {
        let mut array_3 = object.key("Monitors").start_array();
        for item_4 in var_2 {
             {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_monitor::ser_monitor(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.name {
        object.key("Name").string(var_6.as_str());
    }
    Ok(())
}

