// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_queue_hours_of_operation_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateQueueHoursOfOperationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.hours_of_operation_id {
        object.key("HoursOfOperationId").string(var_1.as_str());
    }
    Ok(())
}

