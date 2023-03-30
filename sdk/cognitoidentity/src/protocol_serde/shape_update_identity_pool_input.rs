// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_identity_pool_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateIdentityPoolInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.identity_pool_id {
        object.key("IdentityPoolId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.identity_pool_name {
        object.key("IdentityPoolName").string(var_2.as_str());
    }
     {
        object.key("AllowUnauthenticatedIdentities").boolean(input.allow_unauthenticated_identities);
    }
    if let Some(var_3) = &input.allow_classic_flow {
        object.key("AllowClassicFlow").boolean(*var_3);
    }
    if let Some(var_4) = &input.supported_login_providers {
        #[allow(unused_mut)]
        let mut object_5 = object.key("SupportedLoginProviders").start_object();
        for (key_6, value_7) in var_4 {
             {
                object_5.key(key_6.as_str()).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    if let Some(var_8) = &input.developer_provider_name {
        object.key("DeveloperProviderName").string(var_8.as_str());
    }
    if let Some(var_9) = &input.open_id_connect_provider_ar_ns {
        let mut array_10 = object.key("OpenIdConnectProviderARNs").start_array();
        for item_11 in var_9 {
             {
                array_10.value().string(item_11.as_str());
            }
        }
        array_10.finish();
    }
    if let Some(var_12) = &input.cognito_identity_providers {
        let mut array_13 = object.key("CognitoIdentityProviders").start_array();
        for item_14 in var_12 {
             {
                #[allow(unused_mut)]
                let mut object_15 = array_13.value().start_object();
                crate::protocol_serde::shape_cognito_identity_provider::ser_cognito_identity_provider(&mut object_15, item_14)?;
                object_15.finish();
            }
        }
        array_13.finish();
    }
    if let Some(var_16) = &input.saml_provider_ar_ns {
        let mut array_17 = object.key("SamlProviderARNs").start_array();
        for item_18 in var_16 {
             {
                array_17.value().string(item_18.as_str());
            }
        }
        array_17.finish();
    }
    if let Some(var_19) = &input.identity_pool_tags {
        #[allow(unused_mut)]
        let mut object_20 = object.key("IdentityPoolTags").start_object();
        for (key_21, value_22) in var_19 {
             {
                object_20.key(key_21.as_str()).string(value_22.as_str());
            }
        }
        object_20.finish();
    }
    Ok(())
}

