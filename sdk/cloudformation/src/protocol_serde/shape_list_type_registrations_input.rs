// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_type_registrations_input_input(input: &crate::input::ListTypeRegistrationsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "ListTypeRegistrations", "2010-05-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Type");
    if let Some(var_2) = &input.r#type {
        scope_1.string(var_2.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("TypeName");
    if let Some(var_4) = &input.type_name {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("TypeArn");
    if let Some(var_6) = &input.type_arn {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("RegistrationStatusFilter");
    if let Some(var_8) = &input.registration_status_filter {
        scope_7.string(var_8.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("MaxResults");
    if let Some(var_10) = &input.max_results {
        scope_9.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_10).into()));
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("NextToken");
    if let Some(var_12) = &input.next_token {
        scope_11.string(var_12);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

