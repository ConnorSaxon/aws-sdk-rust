// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_custom_routing_port_mappings_by_destination_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListCustomRoutingPortMappingsByDestinationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.endpoint_id {
        object.key("EndpointId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.destination_address {
        object.key("DestinationAddress").string(var_2.as_str());
    }
    if let Some(var_3) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_3).into()));
    }
    if let Some(var_4) = &input.next_token {
        object.key("NextToken").string(var_4.as_str());
    }
    Ok(())
}

