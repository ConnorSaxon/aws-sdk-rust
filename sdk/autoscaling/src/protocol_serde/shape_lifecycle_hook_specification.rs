// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_lifecycle_hook_specification(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::LifecycleHookSpecification) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("LifecycleHookName");
    if let Some(var_2) = &input.lifecycle_hook_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("LifecycleTransition");
    if let Some(var_4) = &input.lifecycle_transition {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("NotificationMetadata");
    if let Some(var_6) = &input.notification_metadata {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("HeartbeatTimeout");
    if let Some(var_8) = &input.heartbeat_timeout {
        scope_7.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_8).into()));
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("DefaultResult");
    if let Some(var_10) = &input.default_result {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("NotificationTargetARN");
    if let Some(var_12) = &input.notification_target_arn {
        scope_11.string(var_12);
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("RoleARN");
    if let Some(var_14) = &input.role_arn {
        scope_13.string(var_14);
    }
    Ok(())
}

