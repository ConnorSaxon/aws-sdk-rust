// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_snapshot_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteSnapshotInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.snapshot_name {
        object.key("snapshotName").string(var_1.as_str());
    }
    Ok(())
}

