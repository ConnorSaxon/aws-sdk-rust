// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_backend_storage_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateBackendStorageInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.backend_environment_name {
        object.key("backendEnvironmentName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.resource_config {
        #[allow(unused_mut)]
        let mut object_3 = object.key("resourceConfig").start_object();
        crate::protocol_serde::shape_create_backend_storage_resource_config::ser_create_backend_storage_resource_config(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.resource_name {
        object.key("resourceName").string(var_4.as_str());
    }
    Ok(())
}

