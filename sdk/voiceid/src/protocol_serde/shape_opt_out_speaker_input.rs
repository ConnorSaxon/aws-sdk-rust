// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_opt_out_speaker_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::OptOutSpeakerInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.domain_id {
        object.key("DomainId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.speaker_id {
        object.key("SpeakerId").string(var_2.as_str());
    }
    Ok(())
}

