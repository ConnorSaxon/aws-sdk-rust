// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_auto_scaling_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteAutoScalingConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.auto_scaling_configuration_arn {
        object.key("AutoScalingConfigurationArn").string(var_1.as_str());
    }
    Ok(())
}

