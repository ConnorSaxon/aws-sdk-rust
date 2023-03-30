// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_game_server_group_auto_scaling_policy(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::GameServerGroupAutoScalingPolicy) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.estimated_instance_warmup {
        object.key("EstimatedInstanceWarmup").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_1).into()));
    }
    if let Some(var_2) = &input.target_tracking_configuration {
        #[allow(unused_mut)]
        let mut object_3 = object.key("TargetTrackingConfiguration").start_object();
        crate::protocol_serde::shape_target_tracking_configuration::ser_target_tracking_configuration(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}

