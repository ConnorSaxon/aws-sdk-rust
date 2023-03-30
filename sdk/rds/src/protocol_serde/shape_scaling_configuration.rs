// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_scaling_configuration(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::ScalingConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("MinCapacity");
    if let Some(var_2) = &input.min_capacity {
        scope_1.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("MaxCapacity");
    if let Some(var_4) = &input.max_capacity {
        scope_3.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_4).into()));
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("AutoPause");
    if let Some(var_6) = &input.auto_pause {
        scope_5.boolean(*var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("SecondsUntilAutoPause");
    if let Some(var_8) = &input.seconds_until_auto_pause {
        scope_7.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_8).into()));
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("TimeoutAction");
    if let Some(var_10) = &input.timeout_action {
        scope_9.string(var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("SecondsBeforeTimeout");
    if let Some(var_12) = &input.seconds_before_timeout {
        scope_11.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_12).into()));
    }
    Ok(())
}

