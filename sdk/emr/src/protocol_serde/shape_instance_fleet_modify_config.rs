// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_instance_fleet_modify_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::InstanceFleetModifyConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.instance_fleet_id {
        object.key("InstanceFleetId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.target_on_demand_capacity {
        object.key("TargetOnDemandCapacity").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    if let Some(var_3) = &input.target_spot_capacity {
        object.key("TargetSpotCapacity").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_3).into()));
    }
    Ok(())
}

