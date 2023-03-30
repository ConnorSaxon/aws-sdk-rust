// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_stream_configuration_create(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::StreamConfigurationCreate) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.clipboard_mode {
        object.key("clipboardMode").string(var_1.as_str());
    }
    if let Some(var_2) = &input.ec2_instance_types {
        let mut array_3 = object.key("ec2InstanceTypes").start_array();
        for item_4 in var_2 {
             {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.max_session_length_in_minutes {
        object.key("maxSessionLengthInMinutes").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_5).into()));
    }
    if let Some(var_6) = &input.streaming_image_ids {
        let mut array_7 = object.key("streamingImageIds").start_array();
        for item_8 in var_6 {
             {
                array_7.value().string(item_8.as_str());
            }
        }
        array_7.finish();
    }
    if input.max_stopped_session_length_in_minutes != 0 {
        object.key("maxStoppedSessionLengthInMinutes").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_stopped_session_length_in_minutes).into()));
    }
    if let Some(var_9) = &input.session_storage {
        #[allow(unused_mut)]
        let mut object_10 = object.key("sessionStorage").start_object();
        crate::protocol_serde::shape_stream_configuration_session_storage::ser_stream_configuration_session_storage(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.session_backup {
        #[allow(unused_mut)]
        let mut object_12 = object.key("sessionBackup").start_object();
        crate::protocol_serde::shape_stream_configuration_session_backup::ser_stream_configuration_session_backup(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.session_persistence_mode {
        object.key("sessionPersistenceMode").string(var_13.as_str());
    }
    if let Some(var_14) = &input.volume_configuration {
        #[allow(unused_mut)]
        let mut object_15 = object.key("volumeConfiguration").start_object();
        crate::protocol_serde::shape_volume_configuration::ser_volume_configuration(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.automatic_termination_mode {
        object.key("automaticTerminationMode").string(var_16.as_str());
    }
    Ok(())
}

