// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_fhir_datastore_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteFhirDatastoreInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.datastore_id {
        object.key("DatastoreId").string(var_1.as_str());
    }
    Ok(())
}

