// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_backend_storage_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetBackendStorageInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.resource_name {
        object.key("resourceName").string(var_1.as_str());
    }
    Ok(())
}

