// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_scheduled_actions_input_input(input: &crate::input::DescribeScheduledActionsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DescribeScheduledActions", "2011-01-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AutoScalingGroupName");
    if let Some(var_2) = &input.auto_scaling_group_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ScheduledActionNames");
    if let Some(var_4) = &input.scheduled_action_names {
        let mut list_6 = scope_3.start_list(false, None);
        for item_5 in var_4 {
            #[allow(unused_mut)]
            let mut entry_7 = list_6.entry();
            entry_7.string(item_5);
        }
        list_6.finish();
    }
    #[allow(unused_mut)]
    let mut scope_8 = writer.prefix("StartTime");
    if let Some(var_9) = &input.start_time {
        scope_8.date_time(var_9, aws_smithy_types::date_time::Format::DateTime)?;
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("EndTime");
    if let Some(var_11) = &input.end_time {
        scope_10.date_time(var_11, aws_smithy_types::date_time::Format::DateTime)?;
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("NextToken");
    if let Some(var_13) = &input.next_token {
        scope_12.string(var_13);
    }
    #[allow(unused_mut)]
    let mut scope_14 = writer.prefix("MaxRecords");
    if let Some(var_15) = &input.max_records {
        scope_14.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_15).into()));
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

