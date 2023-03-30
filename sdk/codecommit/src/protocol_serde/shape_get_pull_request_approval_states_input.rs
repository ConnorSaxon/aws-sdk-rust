// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_pull_request_approval_states_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetPullRequestApprovalStatesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.pull_request_id {
        object.key("pullRequestId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.revision_id {
        object.key("revisionId").string(var_2.as_str());
    }
    Ok(())
}

