// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_directory_config_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteDirectoryConfigInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.directory_name {
        object.key("DirectoryName").string(var_1.as_str());
    }
    Ok(())
}

