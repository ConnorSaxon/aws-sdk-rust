// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_file_entry(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PutFileEntry) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.file_path {
        object.key("filePath").string(var_1.as_str());
    }
    if let Some(var_2) = &input.file_mode {
        object.key("fileMode").string(var_2.as_str());
    }
    if let Some(var_3) = &input.file_content {
        object.key("fileContent").string_unchecked(&aws_smithy_types::base64::encode(var_3));
    }
    if let Some(var_4) = &input.source_file {
        #[allow(unused_mut)]
        let mut object_5 = object.key("sourceFile").start_object();
        crate::protocol_serde::shape_source_file_specifier::ser_source_file_specifier(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}

