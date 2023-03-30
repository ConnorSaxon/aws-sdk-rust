// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_set_principal_tag_attribute_map_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::SetPrincipalTagAttributeMapInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.identity_provider_name {
        object.key("IdentityProviderName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.use_defaults {
        object.key("UseDefaults").boolean(*var_3);
    }
    if let Some(var_4) = &input.principal_tags {
        #[allow(unused_mut)]
        let mut object_5 = object.key("PrincipalTags").start_object();
        for (key_6, value_7) in var_4 {
             {
                object_5.key(key_6.as_str()).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    Ok(())
}

