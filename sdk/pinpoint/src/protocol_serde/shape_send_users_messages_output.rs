// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_send_users_message_response_payload(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::SendUsersMessageResponse>, crate::error::SendUsersMessagesError> {
    (!body.is_empty()).then(||{
        crate::protocol_serde::shape_send_users_message_response::de_send_users_message_response_payload(body).map_err(crate::error::SendUsersMessagesError::unhandled)
    }).transpose()
}

