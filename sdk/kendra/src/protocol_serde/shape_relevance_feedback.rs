// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_relevance_feedback(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RelevanceFeedback) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.result_id {
        object.key("ResultId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.relevance_value {
        object.key("RelevanceValue").string(var_2.as_str());
    }
    Ok(())
}

