// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_http_endpoint_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::HttpEndpointConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.url {
        object.key("Url").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.access_key {
        object.key("AccessKey").string(var_3.as_str());
    }
    Ok(())
}

