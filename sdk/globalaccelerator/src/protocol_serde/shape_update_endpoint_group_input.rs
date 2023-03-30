// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_endpoint_group_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateEndpointGroupInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.endpoint_group_arn {
        object.key("EndpointGroupArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.endpoint_configurations {
        let mut array_3 = object.key("EndpointConfigurations").start_array();
        for item_4 in var_2 {
             {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_endpoint_configuration::ser_endpoint_configuration(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.traffic_dial_percentage {
        object.key("TrafficDialPercentage").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_6).into()));
    }
    if let Some(var_7) = &input.health_check_port {
        object.key("HealthCheckPort").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_7).into()));
    }
    if let Some(var_8) = &input.health_check_protocol {
        object.key("HealthCheckProtocol").string(var_8.as_str());
    }
    if let Some(var_9) = &input.health_check_path {
        object.key("HealthCheckPath").string(var_9.as_str());
    }
    if let Some(var_10) = &input.health_check_interval_seconds {
        object.key("HealthCheckIntervalSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_10).into()));
    }
    if let Some(var_11) = &input.threshold_count {
        object.key("ThresholdCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_11).into()));
    }
    if let Some(var_12) = &input.port_overrides {
        let mut array_13 = object.key("PortOverrides").start_array();
        for item_14 in var_12 {
             {
                #[allow(unused_mut)]
                let mut object_15 = array_13.value().start_object();
                crate::protocol_serde::shape_port_override::ser_port_override(&mut object_15, item_14)?;
                object_15.finish();
            }
        }
        array_13.finish();
    }
    Ok(())
}

