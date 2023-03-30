// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_db_parameter_groups_input_input(input: &crate::input::DescribeDbParameterGroupsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DescribeDBParameterGroups", "2014-10-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DBParameterGroupName");
    if let Some(var_2) = &input.db_parameter_group_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("Filters");
    if let Some(var_4) = &input.filters {
        let mut list_6 = scope_3.start_list(false, Some("Filter"));
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            crate::protocol_serde::shape_filter::ser_filter(entry_7, item_5)?;
        }
        list_6.finish();
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("MaxRecords");
    if let Some(var_9) = &input.max_records {
        scope_8.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_9).into()));
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("Marker");
    if let Some(var_11) = &input.marker {
        scope_10.string(var_11);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

