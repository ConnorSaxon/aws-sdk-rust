// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_feedback_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeFeedbackInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.insight_id {
        object.key("InsightId").string(var_1.as_str());
    }
    Ok(())
}

