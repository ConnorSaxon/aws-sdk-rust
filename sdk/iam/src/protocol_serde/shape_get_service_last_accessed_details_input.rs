// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_service_last_accessed_details_input_input(input: &crate::input::GetServiceLastAccessedDetailsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "GetServiceLastAccessedDetails", "2010-05-08");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("JobId");
    if let Some(var_2) = &input.job_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("MaxItems");
    if let Some(var_4) = &input.max_items {
        scope_3.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_4).into()));
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Marker");
    if let Some(var_6) = &input.marker {
        scope_5.string(var_6);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

