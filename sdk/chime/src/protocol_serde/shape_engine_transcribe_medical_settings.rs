// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_engine_transcribe_medical_settings(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::EngineTranscribeMedicalSettings) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.language_code {
        object.key("LanguageCode").string(var_1.as_str());
    }
    if let Some(var_2) = &input.specialty {
        object.key("Specialty").string(var_2.as_str());
    }
    if let Some(var_3) = &input.r#type {
        object.key("Type").string(var_3.as_str());
    }
    if let Some(var_4) = &input.vocabulary_name {
        object.key("VocabularyName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.region {
        object.key("Region").string(var_5.as_str());
    }
    if let Some(var_6) = &input.content_identification_type {
        object.key("ContentIdentificationType").string(var_6.as_str());
    }
    Ok(())
}

