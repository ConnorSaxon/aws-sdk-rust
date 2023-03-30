// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_event_subscription_input_input(input: &crate::input::ModifyEventSubscriptionInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "ModifyEventSubscription", "2012-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("SubscriptionName");
    if let Some(var_2) = &input.subscription_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("SnsTopicArn");
    if let Some(var_4) = &input.sns_topic_arn {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("SourceType");
    if let Some(var_6) = &input.source_type {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("SourceIds");
    if let Some(var_8) = &input.source_ids {
        let mut list_10 = scope_7.start_list(false, Some("SourceId"));
        for item_9 in var_8 {
            #[allow(unused_mut)]
            let mut entry_11 = list_10.entry();
            entry_11.string(item_9);
        }
        list_10.finish();
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("EventCategories");
    if let Some(var_13) = &input.event_categories {
        let mut list_15 = scope_12.start_list(false, Some("EventCategory"));
        for item_14 in var_13 {
            #[allow(unused_mut)]
            let mut entry_16 = list_15.entry();
            entry_16.string(item_14);
        }
        list_15.finish();
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("Severity");
    if let Some(var_18) = &input.severity {
        scope_17.string(var_18);
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("Enabled");
    if let Some(var_20) = &input.enabled {
        scope_19.boolean(*var_20);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

