// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_detect_entities_v2_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DetectEntitiesV2Input) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.text {
        object.key("Text").string(var_1.as_str());
    }
    Ok(())
}

