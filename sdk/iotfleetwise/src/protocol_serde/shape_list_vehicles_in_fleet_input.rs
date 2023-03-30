// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_vehicles_in_fleet_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListVehiclesInFleetInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.fleet_id {
        object.key("fleetId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.next_token {
        object.key("nextToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_3).into()));
    }
    Ok(())
}

