// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_decrease_stream_retention_period_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DecreaseStreamRetentionPeriodInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.stream_name {
        object.key("StreamName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.retention_period_hours {
        object.key("RetentionPeriodHours").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    if let Some(var_3) = &input.stream_arn {
        object.key("StreamARN").string(var_3.as_str());
    }
    Ok(())
}

