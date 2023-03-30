// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_cancel_query_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CancelQueryInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.event_data_store {
        object.key("EventDataStore").string(var_1.as_str());
    }
    if let Some(var_2) = &input.query_id {
        object.key("QueryId").string(var_2.as_str());
    }
    Ok(())
}

