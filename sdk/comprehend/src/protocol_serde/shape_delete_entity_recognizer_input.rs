// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_entity_recognizer_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteEntityRecognizerInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.entity_recognizer_arn {
        object.key("EntityRecognizerArn").string(var_1.as_str());
    }
    Ok(())
}

