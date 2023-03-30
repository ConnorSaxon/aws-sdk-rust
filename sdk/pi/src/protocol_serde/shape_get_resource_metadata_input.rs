// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_resource_metadata_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetResourceMetadataInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.service_type {
        object.key("ServiceType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.identifier {
        object.key("Identifier").string(var_2.as_str());
    }
    Ok(())
}

