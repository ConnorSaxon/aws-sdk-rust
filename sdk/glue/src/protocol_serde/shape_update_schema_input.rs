// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_schema_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateSchemaInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.schema_id {
        #[allow(unused_mut)]
        let mut object_2 = object.key("SchemaId").start_object();
        crate::protocol_serde::shape_schema_id::ser_schema_id(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.schema_version_number {
        #[allow(unused_mut)]
        let mut object_4 = object.key("SchemaVersionNumber").start_object();
        crate::protocol_serde::shape_schema_version_number::ser_schema_version_number(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.compatibility {
        object.key("Compatibility").string(var_5.as_str());
    }
    if let Some(var_6) = &input.description {
        object.key("Description").string(var_6.as_str());
    }
    Ok(())
}

