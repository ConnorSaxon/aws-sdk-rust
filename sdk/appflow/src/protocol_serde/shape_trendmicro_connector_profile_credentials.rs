// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_trendmicro_connector_profile_credentials(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::TrendmicroConnectorProfileCredentials) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.api_secret_key {
        object.key("apiSecretKey").string(var_1.as_str());
    }
    Ok(())
}

