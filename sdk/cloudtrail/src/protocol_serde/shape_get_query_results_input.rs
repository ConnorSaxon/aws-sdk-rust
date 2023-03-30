// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_query_results_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetQueryResultsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.event_data_store {
        object.key("EventDataStore").string(var_1.as_str());
    }
    if let Some(var_2) = &input.query_id {
        object.key("QueryId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.next_token {
        object.key("NextToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.max_query_results {
        object.key("MaxQueryResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_4).into()));
    }
    Ok(())
}

