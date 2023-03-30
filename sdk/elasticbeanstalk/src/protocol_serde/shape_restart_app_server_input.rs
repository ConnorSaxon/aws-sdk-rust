// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_restart_app_server_input_input(input: &crate::input::RestartAppServerInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "RestartAppServer", "2010-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("EnvironmentId");
    if let Some(var_2) = &input.environment_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("EnvironmentName");
    if let Some(var_4) = &input.environment_name {
        scope_3.string(var_4);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

