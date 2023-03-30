// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_folder_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateFolderInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.parent_folder_id {
        object.key("ParentFolderId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.resource_state {
        object.key("ResourceState").string(var_3.as_str());
    }
    Ok(())
}

