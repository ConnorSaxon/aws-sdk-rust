// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_source_server_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteSourceServerInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.source_server_id {
        object.key("sourceServerID").string(var_1.as_str());
    }
    Ok(())
}

