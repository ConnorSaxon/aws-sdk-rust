// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_detach_volume_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DetachVolumeInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.volume_arn {
        object.key("VolumeARN").string(var_1.as_str());
    }
    if let Some(var_2) = &input.force_detach {
        object.key("ForceDetach").boolean(*var_2);
    }
    Ok(())
}

