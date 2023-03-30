// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_transit_gateway_attachments_input_input(input: &crate::input::DescribeTransitGatewayAttachmentsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DescribeTransitGatewayAttachments", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("TransitGatewayAttachmentIds");
    if let Some(var_2) = &input.transit_gateway_attachment_ids {
        let mut list_4 = scope_1.start_list(true, None);
        for item_3 in var_2 {
            #[allow(unused_mut)]
            let mut entry_5 = list_4.entry();
            entry_5.string(item_3);
        }
        list_4.finish();
    }
    #[allow(unused_mut)]
    let mut scope_6 = writer.prefix("Filter");
    if let Some(var_7) = &input.filters {
        let mut list_9 = scope_6.start_list(true, Some("Filter"));
        for item_8 in var_7 {
            #[allow(unused_mut)]
            let mut entry_10 = list_9.entry();
            crate::protocol_serde::shape_filter::ser_filter(entry_10, item_8)?;
        }
        list_9.finish();
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("MaxResults");
    if let Some(var_12) = &input.max_results {
        scope_11.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_12).into()));
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("NextToken");
    if let Some(var_14) = &input.next_token {
        scope_13.string(var_14);
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("DryRun");
    if let Some(var_16) = &input.dry_run {
        scope_15.boolean(*var_16);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

