// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_salesforce_connector_profile_credentials(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SalesforceConnectorProfileCredentials) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.access_token {
        object.key("accessToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.refresh_token {
        object.key("refreshToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.o_auth_request {
        #[allow(unused_mut)]
        let mut object_4 = object.key("oAuthRequest").start_object();
        crate::protocol_serde::shape_connector_o_auth_request::ser_connector_o_auth_request(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.client_credentials_arn {
        object.key("clientCredentialsArn").string(var_5.as_str());
    }
    Ok(())
}

