// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_import_api_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ImportApiInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.body {
        object.key("body").string(var_1.as_str());
    }
    Ok(())
}

