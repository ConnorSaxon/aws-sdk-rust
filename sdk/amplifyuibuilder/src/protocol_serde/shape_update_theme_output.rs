// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_entity_payload(body: &[u8]) -> std::result::Result<std::option::Option<crate::model::Theme>, crate::error::UpdateThemeError> {
    (!body.is_empty()).then(||{
        crate::protocol_serde::shape_theme::de_theme_payload(body).map_err(crate::error::UpdateThemeError::unhandled)
    }).transpose()
}

