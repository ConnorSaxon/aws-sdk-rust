// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_marketo_connector_profile_credentials(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MarketoConnectorProfileCredentials) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_id {
        object.key("clientId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_secret {
        object.key("clientSecret").string(var_2.as_str());
    }
    if let Some(var_3) = &input.access_token {
        object.key("accessToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.o_auth_request {
        #[allow(unused_mut)]
        let mut object_5 = object.key("oAuthRequest").start_object();
        crate::protocol_serde::shape_connector_o_auth_request::ser_connector_o_auth_request(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}

