// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_permission_group_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreatePermissionGroupInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.application_permissions {
        let mut array_2 = object.key("applicationPermissions").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.client_token {
        object.key("clientToken").string(var_4.as_str());
    }
    if let Some(var_5) = &input.description {
        object.key("description").string(var_5.as_str());
    }
    if let Some(var_6) = &input.name {
        object.key("name").string(var_6.as_str());
    }
    Ok(())
}

