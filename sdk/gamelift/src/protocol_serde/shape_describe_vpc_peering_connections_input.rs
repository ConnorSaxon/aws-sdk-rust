// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_vpc_peering_connections_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeVpcPeeringConnectionsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.fleet_id {
        object.key("FleetId").string(var_1.as_str());
    }
    Ok(())
}

