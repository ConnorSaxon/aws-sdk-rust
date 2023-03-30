// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_environment_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateEnvironmentInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.apply_during_maintenance_window {
        object.key("applyDuringMaintenanceWindow").boolean(input.apply_during_maintenance_window);
    }
    if let Some(var_1) = &input.desired_capacity {
        object.key("desiredCapacity").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_1).into()));
    }
    if let Some(var_2) = &input.engine_version {
        object.key("engineVersion").string(var_2.as_str());
    }
    if let Some(var_3) = &input.instance_type {
        object.key("instanceType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.preferred_maintenance_window {
        object.key("preferredMaintenanceWindow").string(var_4.as_str());
    }
    Ok(())
}

