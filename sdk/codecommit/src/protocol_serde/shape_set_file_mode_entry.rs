// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_set_file_mode_entry(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SetFileModeEntry) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.file_path {
        object.key("filePath").string(var_1.as_str());
    }
    if let Some(var_2) = &input.file_mode {
        object.key("fileMode").string(var_2.as_str());
    }
    Ok(())
}

