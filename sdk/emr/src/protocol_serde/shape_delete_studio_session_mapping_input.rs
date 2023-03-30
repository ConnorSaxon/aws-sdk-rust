// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_studio_session_mapping_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteStudioSessionMappingInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.studio_id {
        object.key("StudioId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.identity_id {
        object.key("IdentityId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.identity_name {
        object.key("IdentityName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.identity_type {
        object.key("IdentityType").string(var_4.as_str());
    }
    Ok(())
}

