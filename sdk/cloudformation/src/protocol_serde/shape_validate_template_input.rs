// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_validate_template_input_input(input: &crate::input::ValidateTemplateInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "ValidateTemplate", "2010-05-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("TemplateBody");
    if let Some(var_2) = &input.template_body {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("TemplateURL");
    if let Some(var_4) = &input.template_url {
        scope_3.string(var_4);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

