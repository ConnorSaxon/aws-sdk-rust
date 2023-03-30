// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_public_endpoint(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PublicEndpoint) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.address {
        object.key("Address").string(var_1.as_str());
    }
    if let Some(var_2) = &input.attributes {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Attributes").start_object();
        for (key_4, value_5) in var_2 {
             {
                let mut array_6 = object_3.key(key_4.as_str()).start_array();
                for item_7 in value_5 {
                     {
                        array_6.value().string(item_7.as_str());
                    }
                }
                array_6.finish();
            }
        }
        object_3.finish();
    }
    if let Some(var_8) = &input.channel_type {
        object.key("ChannelType").string(var_8.as_str());
    }
    if let Some(var_9) = &input.demographic {
        #[allow(unused_mut)]
        let mut object_10 = object.key("Demographic").start_object();
        crate::protocol_serde::shape_endpoint_demographic::ser_endpoint_demographic(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.effective_date {
        object.key("EffectiveDate").string(var_11.as_str());
    }
    if let Some(var_12) = &input.endpoint_status {
        object.key("EndpointStatus").string(var_12.as_str());
    }
    if let Some(var_13) = &input.location {
        #[allow(unused_mut)]
        let mut object_14 = object.key("Location").start_object();
        crate::protocol_serde::shape_endpoint_location::ser_endpoint_location(&mut object_14, var_13)?;
        object_14.finish();
    }
    if let Some(var_15) = &input.metrics {
        #[allow(unused_mut)]
        let mut object_16 = object.key("Metrics").start_object();
        for (key_17, value_18) in var_15 {
             {
                object_16.key(key_17.as_str()).number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*value_18).into()));
            }
        }
        object_16.finish();
    }
    if let Some(var_19) = &input.opt_out {
        object.key("OptOut").string(var_19.as_str());
    }
    if let Some(var_20) = &input.request_id {
        object.key("RequestId").string(var_20.as_str());
    }
    if let Some(var_21) = &input.user {
        #[allow(unused_mut)]
        let mut object_22 = object.key("User").start_object();
        crate::protocol_serde::shape_endpoint_user::ser_endpoint_user(&mut object_22, var_21)?;
        object_22.finish();
    }
    Ok(())
}

