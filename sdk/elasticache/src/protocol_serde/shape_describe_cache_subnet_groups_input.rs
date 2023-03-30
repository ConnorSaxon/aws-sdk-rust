// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_cache_subnet_groups_input_input(input: &crate::input::DescribeCacheSubnetGroupsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DescribeCacheSubnetGroups", "2015-02-02");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("CacheSubnetGroupName");
    if let Some(var_2) = &input.cache_subnet_group_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("MaxRecords");
    if let Some(var_4) = &input.max_records {
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

