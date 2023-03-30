// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_registry_id(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RegistryId) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.registry_name {
        object.key("RegistryName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.registry_arn {
        object.key("RegistryArn").string(var_2.as_str());
    }
    Ok(())
}

