// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_application_version_input_input(input: &crate::input::UpdateApplicationVersionInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "UpdateApplicationVersion", "2010-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ApplicationName");
    if let Some(var_2) = &input.application_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("VersionLabel");
    if let Some(var_4) = &input.version_label {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Description");
    if let Some(var_6) = &input.description {
        scope_5.string(var_6);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

