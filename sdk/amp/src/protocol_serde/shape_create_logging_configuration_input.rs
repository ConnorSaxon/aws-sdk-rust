// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_logging_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateLoggingConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.log_group_arn {
        object.key("logGroupArn").string(var_2.as_str());
    }
    Ok(())
}

