// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_schema_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteSchemaInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.schema_arn {
        object.key("schemaArn").string(var_1.as_str());
    }
    Ok(())
}

