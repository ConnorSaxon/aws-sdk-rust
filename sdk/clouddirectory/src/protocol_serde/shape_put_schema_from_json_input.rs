// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_schema_from_json_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutSchemaFromJsonInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.document {
        object.key("Document").string(var_1.as_str());
    }
    Ok(())
}

