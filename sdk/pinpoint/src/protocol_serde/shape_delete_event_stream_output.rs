// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_event_stream_payload(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::EventStream>, crate::error::DeleteEventStreamError> {
    (!body.is_empty()).then(||{
        crate::protocol_serde::shape_event_stream::de_event_stream_payload(body).map_err(crate::error::DeleteEventStreamError::unhandled)
    }).transpose()
}

