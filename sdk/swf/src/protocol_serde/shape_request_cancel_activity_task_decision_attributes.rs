// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_request_cancel_activity_task_decision_attributes(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RequestCancelActivityTaskDecisionAttributes) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.activity_id {
        object.key("activityId").string(var_1.as_str());
    }
    Ok(())
}

