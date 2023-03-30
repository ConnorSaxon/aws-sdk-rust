// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_api_key_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateApiKeyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("description").string(var_1.as_str());
    }
    if input.expires != 0 {
        object.key("expires").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.expires).into()));
    }
    Ok(())
}

