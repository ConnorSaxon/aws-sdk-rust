// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_hls_timestamp_range(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::HlsTimestampRange) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.start_timestamp {
        object.key("StartTimestamp").date_time(var_1, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_2) = &input.end_timestamp {
        object.key("EndTimestamp").date_time(var_2, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

