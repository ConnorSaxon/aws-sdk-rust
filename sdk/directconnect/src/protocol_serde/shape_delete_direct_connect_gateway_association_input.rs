// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_direct_connect_gateway_association_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteDirectConnectGatewayAssociationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.association_id {
        object.key("associationId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.direct_connect_gateway_id {
        object.key("directConnectGatewayId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.virtual_gateway_id {
        object.key("virtualGatewayId").string(var_3.as_str());
    }
    Ok(())
}

