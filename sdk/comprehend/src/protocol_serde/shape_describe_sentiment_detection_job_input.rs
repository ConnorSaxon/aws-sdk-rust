// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_sentiment_detection_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeSentimentDetectionJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.job_id {
        object.key("JobId").string(var_1.as_str());
    }
    Ok(())
}

