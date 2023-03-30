// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_template_input_input(input: &crate::input::UpdateTemplateInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "UpdateTemplate", "2010-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Template");
    if let Some(var_2) = &input.template {
        crate::protocol_serde::shape_template::ser_template(scope_1, var_2)?;
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

