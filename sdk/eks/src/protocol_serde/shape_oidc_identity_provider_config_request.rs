// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_oidc_identity_provider_config_request(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::OidcIdentityProviderConfigRequest) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.identity_provider_config_name {
        object.key("identityProviderConfigName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.issuer_url {
        object.key("issuerUrl").string(var_2.as_str());
    }
    if let Some(var_3) = &input.client_id {
        object.key("clientId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.username_claim {
        object.key("usernameClaim").string(var_4.as_str());
    }
    if let Some(var_5) = &input.username_prefix {
        object.key("usernamePrefix").string(var_5.as_str());
    }
    if let Some(var_6) = &input.groups_claim {
        object.key("groupsClaim").string(var_6.as_str());
    }
    if let Some(var_7) = &input.groups_prefix {
        object.key("groupsPrefix").string(var_7.as_str());
    }
    if let Some(var_8) = &input.required_claims {
        #[allow(unused_mut)]
        let mut object_9 = object.key("requiredClaims").start_object();
        for (key_10, value_11) in var_8 {
             {
                object_9.key(key_10.as_str()).string(value_11.as_str());
            }
        }
        object_9.finish();
    }
    Ok(())
}

