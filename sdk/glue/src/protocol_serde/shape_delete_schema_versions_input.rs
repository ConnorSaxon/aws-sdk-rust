// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_schema_versions_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteSchemaVersionsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.schema_id {
        #[allow(unused_mut)]
        let mut object_2 = object.key("SchemaId").start_object();
        crate::protocol_serde::shape_schema_id::ser_schema_id(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.versions {
        object.key("Versions").string(var_3.as_str());
    }
    Ok(())
}

