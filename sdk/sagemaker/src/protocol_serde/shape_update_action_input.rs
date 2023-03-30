// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_action_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateActionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.action_name {
        object.key("ActionName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.status {
        object.key("Status").string(var_3.as_str());
    }
    if let Some(var_4) = &input.properties {
        #[allow(unused_mut)]
        let mut object_5 = object.key("Properties").start_object();
        for (key_6, value_7) in var_4 {
             {
                object_5.key(key_6.as_str()).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    if let Some(var_8) = &input.properties_to_remove {
        let mut array_9 = object.key("PropertiesToRemove").start_array();
        for item_10 in var_8 {
             {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    Ok(())
}

