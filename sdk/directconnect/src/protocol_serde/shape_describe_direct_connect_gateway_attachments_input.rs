// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_direct_connect_gateway_attachments_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeDirectConnectGatewayAttachmentsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.direct_connect_gateway_id {
        object.key("directConnectGatewayId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.virtual_interface_id {
        object.key("virtualInterfaceId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_3).into()));
    }
    if let Some(var_4) = &input.next_token {
        object.key("nextToken").string(var_4.as_str());
    }
    Ok(())
}

