// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_application_snapshot_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteApplicationSnapshotInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.application_name {
        object.key("ApplicationName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.snapshot_name {
        object.key("SnapshotName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.snapshot_creation_timestamp {
        object.key("SnapshotCreationTimestamp").date_time(var_3, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

