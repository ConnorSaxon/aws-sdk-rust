// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_model_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateModelInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.content_type {
        object.key("contentType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.name {
        object.key("name").string(var_3.as_str());
    }
    if let Some(var_4) = &input.schema {
        object.key("schema").string(var_4.as_str());
    }
    Ok(())
}

