// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_notebook_metadata_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetNotebookMetadataInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.notebook_id {
        object.key("NotebookId").string(var_1.as_str());
    }
    Ok(())
}

