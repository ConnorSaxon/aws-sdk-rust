// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disable_serial_console_access_input_input(input: &crate::input::DisableSerialConsoleAccessInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DisableSerialConsoleAccess", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

