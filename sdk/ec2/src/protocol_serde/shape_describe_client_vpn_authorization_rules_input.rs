// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_client_vpn_authorization_rules_input_input(input: &crate::input::DescribeClientVpnAuthorizationRulesInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DescribeClientVpnAuthorizationRules", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ClientVpnEndpointId");
    if let Some(var_2) = &input.client_vpn_endpoint_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("DryRun");
    if let Some(var_4) = &input.dry_run {
        scope_3.boolean(*var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("NextToken");
    if let Some(var_6) = &input.next_token {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("Filter");
    if let Some(var_8) = &input.filters {
        let mut list_10 = scope_7.start_list(true, Some("Filter"));
        for item_9 in var_8 {
            #[allow(unused_mut)]
            let mut entry_11 = list_10.entry();
            crate::protocol_serde::shape_filter::ser_filter(entry_11, item_9)?;
        }
        list_10.finish();
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("MaxResults");
    if let Some(var_13) = &input.max_results {
        scope_12.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_13).into()));
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

