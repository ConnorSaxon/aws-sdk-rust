// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_text_translation_job_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::TextTranslationJobFilter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.job_name {
        object.key("JobName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.job_status {
        object.key("JobStatus").string(var_2.as_str());
    }
    if let Some(var_3) = &input.submitted_before_time {
        object.key("SubmittedBeforeTime").date_time(var_3, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_4) = &input.submitted_after_time {
        object.key("SubmittedAfterTime").date_time(var_4, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    Ok(())
}

