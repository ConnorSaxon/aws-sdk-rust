// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_accept_shared_directory_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AcceptSharedDirectoryInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.shared_directory_id {
        object.key("SharedDirectoryId").string(var_1.as_str());
    }
    Ok(())
}

