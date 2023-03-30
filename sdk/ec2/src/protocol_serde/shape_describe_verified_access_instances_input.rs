// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_verified_access_instances_input_input(input: &crate::input::DescribeVerifiedAccessInstancesInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DescribeVerifiedAccessInstances", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("VerifiedAccessInstanceId");
    if let Some(var_2) = &input.verified_access_instance_ids {
        let mut list_4 = scope_1.start_list(true, Some("item"));
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            entry_5.string(item_3);
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("MaxResults");
    if let Some(var_7) = &input.max_results {
        scope_6.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_7).into()));
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("NextToken");
    if let Some(var_9) = &input.next_token {
        scope_8.string(var_9);
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("Filter");
    if let Some(var_11) = &input.filters {
        let mut list_13 = scope_10.start_list(true, Some("Filter"));
        for item_12 in var_11 {
            #[allow(unused_mut)]
            let mut entry_14 = list_13.entry();
            crate::protocol_serde::shape_filter::ser_filter(entry_14, item_12)?;
        }
        list_13.finish();
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("DryRun");
    if let Some(var_16) = &input.dry_run {
        scope_15.boolean(*var_16);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

