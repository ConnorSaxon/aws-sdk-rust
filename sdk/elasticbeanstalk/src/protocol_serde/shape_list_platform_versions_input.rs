// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_platform_versions_input_input(input: &crate::input::ListPlatformVersionsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "ListPlatformVersions", "2010-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Filters");
    if let Some(var_2) = &input.filters {
        let mut list_4 = scope_1.start_list(false, None);
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            crate::protocol_serde::shape_platform_filter::ser_platform_filter(entry_5, item_3)?;
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("MaxRecords");
    if let Some(var_7) = &input.max_records {
        scope_6.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_7).into()));
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("NextToken");
    if let Some(var_9) = &input.next_token {
        scope_8.string(var_9);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

