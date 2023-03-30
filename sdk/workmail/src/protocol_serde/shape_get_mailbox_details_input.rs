// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_mailbox_details_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetMailboxDetailsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.organization_id {
        object.key("OrganizationId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.user_id {
        object.key("UserId").string(var_2.as_str());
    }
    Ok(())
}

