// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_s3_source_properties(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::S3SourceProperties) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.bucket_name {
        object.key("BucketName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.bucket_prefix {
        object.key("BucketPrefix").string(var_2.as_str());
    }
    Ok(())
}

