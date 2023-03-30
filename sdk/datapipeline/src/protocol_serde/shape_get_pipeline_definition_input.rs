// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_pipeline_definition_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetPipelineDefinitionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.pipeline_id {
        object.key("pipelineId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.version {
        object.key("version").string(var_2.as_str());
    }
    Ok(())
}

