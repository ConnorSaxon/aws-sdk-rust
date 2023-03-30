// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_user_profile_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateUserProfileInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.iam_user_arn {
        object.key("IamUserArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.ssh_username {
        object.key("SshUsername").string(var_2.as_str());
    }
    if let Some(var_3) = &input.ssh_public_key {
        object.key("SshPublicKey").string(var_3.as_str());
    }
    if let Some(var_4) = &input.allow_self_management {
        object.key("AllowSelfManagement").boolean(*var_4);
    }
    Ok(())
}

