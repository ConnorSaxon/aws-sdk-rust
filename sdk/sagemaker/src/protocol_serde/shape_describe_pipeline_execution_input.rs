// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_pipeline_execution_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribePipelineExecutionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.pipeline_execution_arn {
        object.key("PipelineExecutionArn").string(var_1.as_str());
    }
    Ok(())
}

