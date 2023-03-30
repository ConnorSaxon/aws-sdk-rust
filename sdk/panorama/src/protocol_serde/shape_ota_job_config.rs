// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_ota_job_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::OtaJobConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.image_version {
        object.key("ImageVersion").string(var_1.as_str());
    }
    if input.allow_major_version_update {
        object.key("AllowMajorVersionUpdate").boolean(input.allow_major_version_update);
    }
    Ok(())
}

