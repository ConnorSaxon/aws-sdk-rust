// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_custom_data_identifiers_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListCustomDataIdentifiersInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.max_results != 0 {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_results).into()));
    }
    if let Some(var_1) = &input.next_token {
        object.key("nextToken").string(var_1.as_str());
    }
    Ok(())
}

