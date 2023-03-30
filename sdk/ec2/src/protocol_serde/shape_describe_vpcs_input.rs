// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_vpcs_input_input(input: &crate::input::DescribeVpcsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DescribeVpcs", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Filter");
    if let Some(var_2) = &input.filters {
        let mut list_4 = scope_1.start_list(true, Some("Filter"));
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            crate::protocol_serde::shape_filter::ser_filter(entry_5, item_3)?;
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("VpcId");
    if let Some(var_7) = &input.vpc_ids {
        let mut list_9 = scope_6.start_list(true, Some("VpcId"));
        for item_8 in var_7 {
            #[allow(unused_mut)]
            let mut entry_10 = list_9.entry();
            entry_10.string(item_8);
        }
        list_9.finish();
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("DryRun");
    if let Some(var_12) = &input.dry_run {
        scope_11.boolean(*var_12);
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("NextToken");
    if let Some(var_14) = &input.next_token {
        scope_13.string(var_14);
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("MaxResults");
    if let Some(var_16) = &input.max_results {
        scope_15.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_16).into()));
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

