// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_instance_event_window_time_range_request(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::InstanceEventWindowTimeRangeRequest) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("StartWeekDay");
    if let Some(var_2) = &input.start_week_day {
        scope_1.string(var_2.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("StartHour");
    if let Some(var_4) = &input.start_hour {
        scope_3.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_4).into()));
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("EndWeekDay");
    if let Some(var_6) = &input.end_week_day {
        scope_5.string(var_6.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("EndHour");
    if let Some(var_8) = &input.end_hour {
        scope_7.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_8).into()));
    }
    Ok(())
}

