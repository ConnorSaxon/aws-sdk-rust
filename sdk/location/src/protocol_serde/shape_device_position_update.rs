// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_device_position_update(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DevicePositionUpdate) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.device_id {
        object.key("DeviceId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.sample_time {
        object.key("SampleTime").date_time(var_2, aws_smithy_types::date_time::Format::DateTime)?;
    }
    if let Some(var_3) = &input.position {
        let mut array_4 = object.key("Position").start_array();
        for item_5 in var_3 {
             {
                array_4.value().number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*item_5).into()));
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.accuracy {
        #[allow(unused_mut)]
        let mut object_7 = object.key("Accuracy").start_object();
        crate::protocol_serde::shape_positional_accuracy::ser_positional_accuracy(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.position_properties {
        #[allow(unused_mut)]
        let mut object_9 = object.key("PositionProperties").start_object();
        for (key_10, value_11) in var_8 {
             {
                object_9.key(key_10.as_str()).string(value_11.as_str());
            }
        }
        object_9.finish();
    }
    Ok(())
}

