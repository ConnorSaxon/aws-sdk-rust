// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_join_storage_session_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::JoinStorageSessionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.channel_arn {
        object.key("channelArn").string(var_1.as_str());
    }
    Ok(())
}

