// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_wireless_gateway_task_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateWirelessGatewayTaskInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.wireless_gateway_task_definition_id {
        object.key("WirelessGatewayTaskDefinitionId").string(var_1.as_str());
    }
    Ok(())
}

