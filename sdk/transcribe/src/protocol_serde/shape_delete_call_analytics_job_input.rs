// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_call_analytics_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteCallAnalyticsJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.call_analytics_job_name {
        object.key("CallAnalyticsJobName").string(var_1.as_str());
    }
    Ok(())
}

