// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_crawlers_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetCrawlersInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_1).into()));
    }
    if let Some(var_2) = &input.next_token {
        object.key("NextToken").string(var_2.as_str());
    }
    Ok(())
}

