// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_medical_vocabulary_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateMedicalVocabularyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.vocabulary_name {
        object.key("VocabularyName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.language_code {
        object.key("LanguageCode").string(var_2.as_str());
    }
    if let Some(var_3) = &input.vocabulary_file_uri {
        object.key("VocabularyFileUri").string(var_3.as_str());
    }
    Ok(())
}

