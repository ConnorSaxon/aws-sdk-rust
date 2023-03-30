// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_studio_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateStudioInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.admin_role_arn {
        object.key("adminRoleArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.display_name {
        object.key("displayName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.user_role_arn {
        object.key("userRoleArn").string(var_3.as_str());
    }
    Ok(())
}

