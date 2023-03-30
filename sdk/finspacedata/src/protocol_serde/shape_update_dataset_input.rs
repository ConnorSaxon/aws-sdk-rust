// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_dataset_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateDatasetInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.alias {
        object.key("alias").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_token {
        object.key("clientToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.dataset_description {
        object.key("datasetDescription").string(var_3.as_str());
    }
    if let Some(var_4) = &input.dataset_title {
        object.key("datasetTitle").string(var_4.as_str());
    }
    if let Some(var_5) = &input.kind {
        object.key("kind").string(var_5.as_str());
    }
    if let Some(var_6) = &input.schema_definition {
        #[allow(unused_mut)]
        let mut object_7 = object.key("schemaDefinition").start_object();
        crate::protocol_serde::shape_schema_union::ser_schema_union(&mut object_7, var_6)?;
        object_7.finish();
    }
    Ok(())
}

