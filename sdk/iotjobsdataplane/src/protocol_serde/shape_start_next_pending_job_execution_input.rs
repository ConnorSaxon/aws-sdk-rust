// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_next_pending_job_execution_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartNextPendingJobExecutionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.status_details {
        #[allow(unused_mut)]
        let mut object_2 = object.key("statusDetails").start_object();
        for (key_3, value_4) in var_1 {
             {
                object_2.key(key_3.as_str()).string(value_4.as_str());
            }
        }
        object_2.finish();
    }
    if let Some(var_5) = &input.step_timeout_in_minutes {
        object.key("stepTimeoutInMinutes").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_5).into()));
    }
    Ok(())
}

