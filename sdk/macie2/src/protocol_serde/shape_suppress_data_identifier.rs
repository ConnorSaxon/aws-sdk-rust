// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_suppress_data_identifier(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SuppressDataIdentifier) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.id {
        object.key("id").string(var_1.as_str());
    }
    if let Some(var_2) = &input.r#type {
        object.key("type").string(var_2.as_str());
    }
    Ok(())
}

