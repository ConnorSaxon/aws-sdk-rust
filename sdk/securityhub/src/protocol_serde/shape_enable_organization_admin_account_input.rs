// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_enable_organization_admin_account_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::EnableOrganizationAdminAccountInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.admin_account_id {
        object.key("AdminAccountId").string(var_1.as_str());
    }
    Ok(())
}

