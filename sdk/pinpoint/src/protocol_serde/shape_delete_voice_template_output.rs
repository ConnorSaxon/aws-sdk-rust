// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_message_body_payload(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::MessageBody>, crate::error::DeleteVoiceTemplateError> {
    (!body.is_empty()).then(||{
        crate::protocol_serde::shape_message_body::de_message_body_payload(body).map_err(crate::error::DeleteVoiceTemplateError::unhandled)
    }).transpose()
}

