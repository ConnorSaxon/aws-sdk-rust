// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_export_read_set(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ExportReadSet) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.read_set_id {
        object.key("readSetId").string(var_1.as_str());
    }
    Ok(())
}

