// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_create_template_message_body_payload(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::CreateTemplateMessageBody>, crate::error::CreateVoiceTemplateError> {
    (!body.is_empty()).then(||{
        crate::protocol_serde::shape_create_template_message_body::de_create_template_message_body_payload(body).map_err(crate::error::CreateVoiceTemplateError::unhandled)
    }).transpose()
}

