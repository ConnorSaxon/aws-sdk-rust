// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disable_ldaps_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DisableLdapsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.directory_id {
        object.key("DirectoryId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.r#type {
        object.key("Type").string(var_2.as_str());
    }
    Ok(())
}

