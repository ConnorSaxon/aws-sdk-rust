// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_reset_alarm_action_request(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ResetAlarmActionRequest) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.request_id {
        object.key("requestId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.alarm_model_name {
        object.key("alarmModelName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.key_value {
        object.key("keyValue").string(var_3.as_str());
    }
    if let Some(var_4) = &input.note {
        object.key("note").string(var_4.as_str());
    }
    Ok(())
}

