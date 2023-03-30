// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_apps_lists_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListAppsListsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.default_lists {
        object.key("DefaultLists").boolean(input.default_lists);
    }
    if let Some(var_1) = &input.next_token {
        object.key("NextToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    Ok(())
}

