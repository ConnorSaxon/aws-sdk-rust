// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_volume_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteVolumeInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.volume_arn {
        object.key("VolumeARN").string(var_1.as_str());
    }
    Ok(())
}

