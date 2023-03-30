// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_deregister_task_from_maintenance_window_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeregisterTaskFromMaintenanceWindowInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.window_id {
        object.key("WindowId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.window_task_id {
        object.key("WindowTaskId").string(var_2.as_str());
    }
    Ok(())
}

