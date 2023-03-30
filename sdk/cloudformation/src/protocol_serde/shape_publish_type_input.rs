// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_publish_type_input_input(input: &crate::input::PublishTypeInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "PublishType", "2010-05-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Type");
    if let Some(var_2) = &input.r#type {
        scope_1.string(var_2.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Arn");
    if let Some(var_4) = &input.arn {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("TypeName");
    if let Some(var_6) = &input.type_name {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("PublicVersionNumber");
    if let Some(var_8) = &input.public_version_number {
        scope_7.string(var_8);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

