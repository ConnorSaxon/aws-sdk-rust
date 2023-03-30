// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_vocabularies_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListVocabulariesInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.next_token {
        object.key("NextToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    if let Some(var_3) = &input.state_equals {
        object.key("StateEquals").string(var_3.as_str());
    }
    if let Some(var_4) = &input.name_contains {
        object.key("NameContains").string(var_4.as_str());
    }
    Ok(())
}

