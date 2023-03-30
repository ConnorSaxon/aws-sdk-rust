// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_db_proxy_target_groups_input_input(input: &crate::input::DescribeDbProxyTargetGroupsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DescribeDBProxyTargetGroups", "2014-10-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DBProxyName");
    if let Some(var_2) = &input.db_proxy_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("TargetGroupName");
    if let Some(var_4) = &input.target_group_name {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("Filters");
    if let Some(var_6) = &input.filters {
        let mut list_8 = scope_5.start_list(false, Some("Filter"));
        for item_7 in var_6 {
            #[allow(unused_mut)]
            let mut entry_9 = list_8.entry();
            crate::protocol_serde::shape_filter::ser_filter(entry_9, item_7)?;
        }
        list_8.finish();
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("Marker");
    if let Some(var_11) = &input.marker {
        scope_10.string(var_11);
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("MaxRecords");
    if let Some(var_13) = &input.max_records {
        scope_12.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_13).into()));
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

