// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disable_gateway_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DisableGatewayInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.gateway_arn {
        object.key("GatewayARN").string(var_1.as_str());
    }
    Ok(())
}

