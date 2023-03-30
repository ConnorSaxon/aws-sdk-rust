// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_configuration_set_sending_enabled_input_input(input: &crate::input::UpdateConfigurationSetSendingEnabledInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "UpdateConfigurationSetSendingEnabled", "2010-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ConfigurationSetName");
    if let Some(var_2) = &input.configuration_set_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Enabled");
     {
        scope_3.boolean(input.enabled);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

