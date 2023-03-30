// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_predictive_scaling_forecast_input_input(input: &crate::input::GetPredictiveScalingForecastInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "GetPredictiveScalingForecast", "2011-01-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AutoScalingGroupName");
    if let Some(var_2) = &input.auto_scaling_group_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("PolicyName");
    if let Some(var_4) = &input.policy_name {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("StartTime");
    if let Some(var_6) = &input.start_time {
        scope_5.date_time(var_6, aws_smithy_types::date_time::Format::DateTime)?;
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("EndTime");
    if let Some(var_8) = &input.end_time {
        scope_7.date_time(var_8, aws_smithy_types::date_time::Format::DateTime)?;
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

