// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_data_retention_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateDataRetentionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.current_version {
        object.key("CurrentVersion").string(var_1.as_str());
    }
    if let Some(var_2) = &input.data_retention_change_in_hours {
        object.key("DataRetentionChangeInHours").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    if let Some(var_3) = &input.operation {
        object.key("Operation").string(var_3.as_str());
    }
    if let Some(var_4) = &input.stream_arn {
        object.key("StreamARN").string(var_4.as_str());
    }
    if let Some(var_5) = &input.stream_name {
        object.key("StreamName").string(var_5.as_str());
    }
    Ok(())
}

