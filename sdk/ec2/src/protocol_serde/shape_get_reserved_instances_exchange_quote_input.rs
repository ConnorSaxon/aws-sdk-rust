// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_reserved_instances_exchange_quote_input_input(input: &crate::input::GetReservedInstancesExchangeQuoteInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "GetReservedInstancesExchangeQuote", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DryRun");
    if let Some(var_2) = &input.dry_run {
        scope_1.boolean(*var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ReservedInstanceId");
    if let Some(var_4) = &input.reserved_instance_ids {
        let mut list_6 = scope_3.start_list(true, Some("ReservedInstanceId"));
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            entry_7.string(item_5);
        }
        list_6.finish();
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("TargetConfiguration");
    if let Some(var_9) = &input.target_configurations {
        let mut list_11 = scope_8.start_list(true, Some("TargetConfigurationRequest"));
        for item_10 in var_9 {
            #[allow(unused_mut)]
            let mut entry_12 = list_11.entry();
            crate::protocol_serde::shape_target_configuration_request::ser_target_configuration_request(entry_12, item_10)?;
        }
        list_11.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

