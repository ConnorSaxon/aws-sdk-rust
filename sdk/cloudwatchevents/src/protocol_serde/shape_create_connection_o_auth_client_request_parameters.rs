// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_connection_o_auth_client_request_parameters(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::CreateConnectionOAuthClientRequestParameters) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_id {
        object.key("ClientID").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_secret {
        object.key("ClientSecret").string(var_2.as_str());
    }
    Ok(())
}

