// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_participant_details(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ParticipantDetails) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.display_name {
        object.key("DisplayName").string(var_1.as_str());
    }
    Ok(())
}

