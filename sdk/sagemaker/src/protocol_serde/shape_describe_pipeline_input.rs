// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_pipeline_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribePipelineInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.pipeline_name {
        object.key("PipelineName").string(var_1.as_str());
    }
    Ok(())
}

