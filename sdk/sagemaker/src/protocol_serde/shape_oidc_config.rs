// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_oidc_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::OidcConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_id {
        object.key("ClientId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_secret {
        object.key("ClientSecret").string(var_2.as_str());
    }
    if let Some(var_3) = &input.issuer {
        object.key("Issuer").string(var_3.as_str());
    }
    if let Some(var_4) = &input.authorization_endpoint {
        object.key("AuthorizationEndpoint").string(var_4.as_str());
    }
    if let Some(var_5) = &input.token_endpoint {
        object.key("TokenEndpoint").string(var_5.as_str());
    }
    if let Some(var_6) = &input.user_info_endpoint {
        object.key("UserInfoEndpoint").string(var_6.as_str());
    }
    if let Some(var_7) = &input.logout_endpoint {
        object.key("LogoutEndpoint").string(var_7.as_str());
    }
    if let Some(var_8) = &input.jwks_uri {
        object.key("JwksUri").string(var_8.as_str());
    }
    Ok(())
}

