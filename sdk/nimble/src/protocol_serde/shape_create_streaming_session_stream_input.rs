// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_streaming_session_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateStreamingSessionStreamInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.expiration_in_seconds {
        object.key("expirationInSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_1).into()));
    }
    Ok(())
}

