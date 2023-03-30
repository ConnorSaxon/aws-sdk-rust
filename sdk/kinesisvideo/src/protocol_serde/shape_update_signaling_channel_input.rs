// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_signaling_channel_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateSignalingChannelInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.channel_arn {
        object.key("ChannelARN").string(var_1.as_str());
    }
    if let Some(var_2) = &input.current_version {
        object.key("CurrentVersion").string(var_2.as_str());
    }
    if let Some(var_3) = &input.single_master_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("SingleMasterConfiguration").start_object();
        crate::protocol_serde::shape_single_master_configuration::ser_single_master_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

