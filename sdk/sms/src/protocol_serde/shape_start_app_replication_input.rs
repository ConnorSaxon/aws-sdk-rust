// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_app_replication_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartAppReplicationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.app_id {
        object.key("appId").string(var_1.as_str());
    }
    Ok(())
}

