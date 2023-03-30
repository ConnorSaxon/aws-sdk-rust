// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_journey_state_request_http_payload(payload: &std::option::Option<crate::model::JourneyStateRequest>) -> Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::BuildError> {
    let payload = match payload.as_ref() {
                                Some(t) => t,
                                None => return Ok(
        crate::protocol_serde::rest_json_unsetpayload()
    )};
    Ok(
        crate::protocol_serde::shape_update_journey_state_input::ser_journey_state_request_payload(payload)?
    )
}

pub fn ser_journey_state_request_payload(input: &crate::model::JourneyStateRequest) -> std::result::Result<std::vec::Vec<u8>, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_journey_state_request::ser_journey_state_request(&mut object, input)?;
    object.finish();
    Ok(out.into_bytes())
}

