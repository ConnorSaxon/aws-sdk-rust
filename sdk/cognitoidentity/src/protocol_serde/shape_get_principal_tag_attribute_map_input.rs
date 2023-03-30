// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_principal_tag_attribute_map_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetPrincipalTagAttributeMapInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.identity_provider_name {
        object.key("IdentityProviderName").string(var_2.as_str());
    }
    Ok(())
}

