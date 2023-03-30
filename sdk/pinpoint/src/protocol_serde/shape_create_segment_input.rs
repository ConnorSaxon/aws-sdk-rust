// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_write_segment_request_http_payload(payload: &std::option::Option<crate::model::WriteSegmentRequest>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::protocol_serde::rest_json_unsetpayload()
    )};
    Ok(
        crate::protocol_serde::shape_create_segment_input::ser_write_segment_request_payload(payload)?
    )
}

pub fn ser_write_segment_request_payload(input: &crate::model::WriteSegmentRequest) -> std::result::Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_write_segment_request::ser_write_segment_request(&mut object, input)?;
    object.finish();
    Ok(out.into_bytes())
}

