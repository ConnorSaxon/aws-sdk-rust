// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_retention_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutRetentionConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
     {
        object.key("RetentionPeriodInDays").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.retention_period_in_days).into()));
    }
    Ok(())
}

