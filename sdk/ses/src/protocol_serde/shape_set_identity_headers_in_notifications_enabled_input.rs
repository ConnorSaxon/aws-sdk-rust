// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_set_identity_headers_in_notifications_enabled_input_input(input: &crate::input::SetIdentityHeadersInNotificationsEnabledInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "SetIdentityHeadersInNotificationsEnabled", "2010-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Identity");
    if let Some(var_2) = &input.identity {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("NotificationType");
    if let Some(var_4) = &input.notification_type {
        scope_3.string(var_4.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Enabled");
     {
        scope_5.boolean(input.enabled);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

