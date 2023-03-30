// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_slot_start_time_range_request(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::SlotStartTimeRangeRequest) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("EarliestTime");
    if let Some(var_2) = &input.earliest_time {
        scope_1.date_time(var_2, aws_smithy_types::date_time::Format::DateTime)?;
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("LatestTime");
    if let Some(var_4) = &input.latest_time {
        scope_3.date_time(var_4, aws_smithy_types::date_time::Format::DateTime)?;
    }
    Ok(())
}

