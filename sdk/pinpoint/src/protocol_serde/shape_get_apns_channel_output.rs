// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_apns_channel_response_payload(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::ApnsChannelResponse>, crate::error::GetApnsChannelError> {
    (!body.is_empty()).then(||{
        crate::protocol_serde::shape_apns_channel_response::de_apns_channel_response_payload(body).map_err(crate::error::GetApnsChannelError::unhandled)
    }).transpose()
}

