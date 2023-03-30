// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_channel_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateChannelInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.metadata {
        object.key("Metadata").string(var_1.as_str());
    }
    if let Some(var_2) = &input.mode {
        object.key("Mode").string(var_2.as_str());
    }
    if let Some(var_3) = &input.name {
        object.key("Name").string(var_3.as_str());
    }
    Ok(())
}

