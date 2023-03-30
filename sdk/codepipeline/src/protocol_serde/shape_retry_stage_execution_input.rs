// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_retry_stage_execution_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RetryStageExecutionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.pipeline_name {
        object.key("pipelineName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.stage_name {
        object.key("stageName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.pipeline_execution_id {
        object.key("pipelineExecutionId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.retry_mode {
        object.key("retryMode").string(var_4.as_str());
    }
    Ok(())
}

