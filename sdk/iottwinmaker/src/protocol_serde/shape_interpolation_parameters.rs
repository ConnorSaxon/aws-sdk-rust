// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_interpolation_parameters(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::InterpolationParameters) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.interpolation_type {
        object.key("interpolationType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.interval_in_seconds {
        object.key("intervalInSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    Ok(())
}

