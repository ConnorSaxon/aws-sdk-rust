// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_unlink_developer_identity_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UnlinkDeveloperIdentityInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.identity_id {
        object.key("IdentityId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.developer_provider_name {
        object.key("DeveloperProviderName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.developer_user_identifier {
        object.key("DeveloperUserIdentifier").string(var_4.as_str());
    }
    Ok(())
}

