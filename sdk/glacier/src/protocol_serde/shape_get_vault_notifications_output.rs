// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_vault_notification_config_payload(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::VaultNotificationConfig>, crate::error::GetVaultNotificationsError> {
    (!body.is_empty()).then(||{
        crate::protocol_serde::shape_vault_notification_config::de_vault_notification_config_payload(body).map_err(crate::error::GetVaultNotificationsError::unhandled)
    }).transpose()
}

