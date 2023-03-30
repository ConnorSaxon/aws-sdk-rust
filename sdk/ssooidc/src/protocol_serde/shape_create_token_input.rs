// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_token_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateTokenInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_id {
        object.key("clientId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_secret {
        object.key("clientSecret").string(var_2.as_str());
    }
    if let Some(var_3) = &input.code {
        object.key("code").string(var_3.as_str());
    }
    if let Some(var_4) = &input.device_code {
        object.key("deviceCode").string(var_4.as_str());
    }
    if let Some(var_5) = &input.grant_type {
        object.key("grantType").string(var_5.as_str());
    }
    if let Some(var_6) = &input.redirect_uri {
        object.key("redirectUri").string(var_6.as_str());
    }
    if let Some(var_7) = &input.refresh_token {
        object.key("refreshToken").string(var_7.as_str());
    }
    if let Some(var_8) = &input.scope {
        let mut array_9 = object.key("scope").start_array();
        for item_10 in var_8 {
             {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    Ok(())
}

