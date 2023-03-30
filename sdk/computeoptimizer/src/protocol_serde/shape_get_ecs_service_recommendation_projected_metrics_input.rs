// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_ecs_service_recommendation_projected_metrics_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetEcsServiceRecommendationProjectedMetricsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.service_arn {
        object.key("serviceArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.stat {
        object.key("stat").string(var_2.as_str());
    }
     {
        object.key("period").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.period).into()));
    }
    if let Some(var_3) = &input.start_time {
        object.key("startTime").date_time(var_3, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_4) = &input.end_time {
        object.key("endTime").date_time(var_4, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

