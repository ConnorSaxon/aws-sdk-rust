// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_file_cache_lustre_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::UpdateFileCacheLustreConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.weekly_maintenance_start_time {
        object.key("WeeklyMaintenanceStartTime").string(var_1.as_str());
    }
    Ok(())
}

