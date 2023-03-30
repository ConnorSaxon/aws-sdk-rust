// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_s3_object(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::S3Object) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.bucket {
        object.key("Bucket").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.version {
        object.key("Version").string(var_3.as_str());
    }
    Ok(())
}

