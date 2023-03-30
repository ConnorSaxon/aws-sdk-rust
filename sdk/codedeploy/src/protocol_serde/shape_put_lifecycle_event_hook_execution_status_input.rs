// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_lifecycle_event_hook_execution_status_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutLifecycleEventHookExecutionStatusInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.deployment_id {
        object.key("deploymentId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.lifecycle_event_hook_execution_id {
        object.key("lifecycleEventHookExecutionId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.status {
        object.key("status").string(var_3.as_str());
    }
    Ok(())
}

