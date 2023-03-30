// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_executors_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListExecutorsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.session_id {
        object.key("SessionId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.executor_state_filter {
        object.key("ExecutorStateFilter").string(var_2.as_str());
    }
    if let Some(var_3) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_3).into()));
    }
    if let Some(var_4) = &input.next_token {
        object.key("NextToken").string(var_4.as_str());
    }
    Ok(())
}

