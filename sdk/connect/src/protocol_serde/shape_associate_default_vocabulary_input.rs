// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_associate_default_vocabulary_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AssociateDefaultVocabularyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.vocabulary_id {
        object.key("VocabularyId").string(var_1.as_str());
    }
    Ok(())
}

