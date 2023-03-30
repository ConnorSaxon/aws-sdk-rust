// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_network_profile_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateNetworkProfileInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.project_arn {
        object.key("projectArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.description {
        object.key("description").string(var_3.as_str());
    }
    if let Some(var_4) = &input.r#type {
        object.key("type").string(var_4.as_str());
    }
    if let Some(var_5) = &input.uplink_bandwidth_bits {
        object.key("uplinkBandwidthBits").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_5).into()));
    }
    if let Some(var_6) = &input.downlink_bandwidth_bits {
        object.key("downlinkBandwidthBits").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_6).into()));
    }
    if let Some(var_7) = &input.uplink_delay_ms {
        object.key("uplinkDelayMs").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_7).into()));
    }
    if let Some(var_8) = &input.downlink_delay_ms {
        object.key("downlinkDelayMs").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_8).into()));
    }
    if let Some(var_9) = &input.uplink_jitter_ms {
        object.key("uplinkJitterMs").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_9).into()));
    }
    if let Some(var_10) = &input.downlink_jitter_ms {
        object.key("downlinkJitterMs").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_10).into()));
    }
    if input.uplink_loss_percent != 0 {
        object.key("uplinkLossPercent").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.uplink_loss_percent).into()));
    }
    if input.downlink_loss_percent != 0 {
        object.key("downlinkLossPercent").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.downlink_loss_percent).into()));
    }
    Ok(())
}

