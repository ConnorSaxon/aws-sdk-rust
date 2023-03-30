// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_relational_database_log_events_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetRelationalDatabaseLogEventsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.relational_database_name {
        object.key("relationalDatabaseName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.log_stream_name {
        object.key("logStreamName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.start_time {
        object.key("startTime").date_time(var_3, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_4) = &input.end_time {
        object.key("endTime").date_time(var_4, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_5) = &input.start_from_head {
        object.key("startFromHead").boolean(*var_5);
    }
    if let Some(var_6) = &input.page_token {
        object.key("pageToken").string(var_6.as_str());
    }
    Ok(())
}

