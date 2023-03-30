// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_activate_device_identifier_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ActivateDeviceIdentifierInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.device_identifier_arn {
        object.key("deviceIdentifierArn").string(var_2.as_str());
    }
    Ok(())
}

