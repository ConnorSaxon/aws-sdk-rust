// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_register_cross_account_access_role_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RegisterCrossAccountAccessRoleInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.role_arn {
        object.key("roleArn").string(var_1.as_str());
    }
    Ok(())
}

