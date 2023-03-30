// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_workgroups_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListWorkgroupsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.next_token {
        object.key("nextToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.max_results {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    Ok(())
}

