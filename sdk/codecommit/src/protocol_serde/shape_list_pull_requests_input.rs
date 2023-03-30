// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_pull_requests_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListPullRequestsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.repository_name {
        object.key("repositoryName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.author_arn {
        object.key("authorArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.pull_request_status {
        object.key("pullRequestStatus").string(var_3.as_str());
    }
    if let Some(var_4) = &input.next_token {
        object.key("nextToken").string(var_4.as_str());
    }
    if let Some(var_5) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_5).into()));
    }
    Ok(())
}

