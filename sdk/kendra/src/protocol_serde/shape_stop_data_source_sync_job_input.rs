// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_stop_data_source_sync_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StopDataSourceSyncJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.id {
        object.key("Id").string(var_1.as_str());
    }
    if let Some(var_2) = &input.index_id {
        object.key("IndexId").string(var_2.as_str());
    }
    Ok(())
}

