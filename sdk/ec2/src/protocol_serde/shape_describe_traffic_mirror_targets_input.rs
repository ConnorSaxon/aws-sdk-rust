// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_traffic_mirror_targets_input_input(input: &crate::input::DescribeTrafficMirrorTargetsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DescribeTrafficMirrorTargets", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("TrafficMirrorTargetId");
    if let Some(var_2) = &input.traffic_mirror_target_ids {
        let mut list_4 = scope_1.start_list(true, Some("item"));
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            entry_5.string(item_3);
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("DryRun");
    if let Some(var_7) = &input.dry_run {
        scope_6.boolean(*var_7);
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("Filter");
    if let Some(var_9) = &input.filters {
        let mut list_11 = scope_8.start_list(true, Some("Filter"));
        for item_10 in var_9 {
            #[allow(unused_mut)]
            let mut entry_12 = list_11.entry();
            crate::protocol_serde::shape_filter::ser_filter(entry_12, item_10)?;
        }
        list_11.finish();
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("MaxResults");
    if let Some(var_14) = &input.max_results {
        scope_13.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_14).into()));
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("NextToken");
    if let Some(var_16) = &input.next_token {
        scope_15.string(var_16);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

