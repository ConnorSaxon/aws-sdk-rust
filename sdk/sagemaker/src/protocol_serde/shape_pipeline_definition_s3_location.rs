// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_pipeline_definition_s3_location(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PipelineDefinitionS3Location) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.bucket {
        object.key("Bucket").string(var_1.as_str());
    }
    if let Some(var_2) = &input.object_key {
        object.key("ObjectKey").string(var_2.as_str());
    }
    if let Some(var_3) = &input.version_id {
        object.key("VersionId").string(var_3.as_str());
    }
    Ok(())
}

