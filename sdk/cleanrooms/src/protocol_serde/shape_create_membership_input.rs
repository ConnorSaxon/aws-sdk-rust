// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_membership_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateMembershipInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.collaboration_identifier {
        object.key("collaborationIdentifier").string(var_1.as_str());
    }
    if let Some(var_2) = &input.query_log_status {
        object.key("queryLogStatus").string(var_2.as_str());
    }
    Ok(())
}

