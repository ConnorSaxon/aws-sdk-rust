// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_known_host_keys_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteKnownHostKeysInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.instance_name {
        object.key("instanceName").string(var_1.as_str());
    }
    Ok(())
}

