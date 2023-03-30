// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_ml_labeling_set_generation_task_run_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartMlLabelingSetGenerationTaskRunInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.transform_id {
        object.key("TransformId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.output_s3_path {
        object.key("OutputS3Path").string(var_2.as_str());
    }
    Ok(())
}

