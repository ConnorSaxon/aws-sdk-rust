// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_calculate_route_car_mode_options(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::CalculateRouteCarModeOptions) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.avoid_ferries {
        object.key("AvoidFerries").boolean(*var_1);
    }
    if let Some(var_2) = &input.avoid_tolls {
        object.key("AvoidTolls").boolean(*var_2);
    }
    Ok(())
}

