// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_instance_profiles_input_input(input: &crate::input::ListInstanceProfilesInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "ListInstanceProfiles", "2010-05-08");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("PathPrefix");
    if let Some(var_2) = &input.path_prefix {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Marker");
    if let Some(var_4) = &input.marker {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("MaxItems");
    if let Some(var_6) = &input.max_items {
        scope_5.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_6).into()));
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

