// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_logging_options_payload_http_payload(payload: &std::option::Option<crate::model::LoggingOptionsPayload>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::protocol_serde::rest_json_unsetpayload()
    )};
    Ok(
        crate::protocol_serde::shape_set_logging_options_input::ser_logging_options_payload_payload(payload)?
    )
}

pub fn ser_logging_options_payload_payload(input: &crate::model::LoggingOptionsPayload) -> std::result::Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_logging_options_payload::ser_logging_options_payload(&mut object, input)?;
    object.finish();
    Ok(out.into_bytes())
}

