// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_workspace_state_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ModifyWorkspaceStateInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.workspace_id {
        object.key("WorkspaceId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.workspace_state {
        object.key("WorkspaceState").string(var_2.as_str());
    }
    Ok(())
}

