// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_new_launch_profile_member(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::NewLaunchProfileMember) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.persona {
        object.key("persona").string(var_1.as_str());
    }
    if let Some(var_2) = &input.principal_id {
        object.key("principalId").string(var_2.as_str());
    }
    Ok(())
}

