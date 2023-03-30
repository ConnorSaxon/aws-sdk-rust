// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_log_patterns_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListLogPatternsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.pattern_set_name {
        object.key("PatternSetName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_3).into()));
    }
    if let Some(var_4) = &input.next_token {
        object.key("NextToken").string(var_4.as_str());
    }
    Ok(())
}

