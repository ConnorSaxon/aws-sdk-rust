// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_retention_policy(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RetentionPolicy) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.home_efs_file_system {
        object.key("HomeEfsFileSystem").string(var_1.as_str());
    }
    Ok(())
}

