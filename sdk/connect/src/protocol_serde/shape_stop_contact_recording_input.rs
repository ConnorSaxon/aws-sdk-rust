// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_stop_contact_recording_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StopContactRecordingInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.contact_id {
        object.key("ContactId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.initial_contact_id {
        object.key("InitialContactId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.instance_id {
        object.key("InstanceId").string(var_3.as_str());
    }
    Ok(())
}

