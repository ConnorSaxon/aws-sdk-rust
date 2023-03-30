// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_backend_storage_resource_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::CreateBackendStorageResourceConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.bucket_name {
        object.key("bucketName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.permissions {
        #[allow(unused_mut)]
        let mut object_3 = object.key("permissions").start_object();
        crate::protocol_serde::shape_backend_storage_permissions::ser_backend_storage_permissions(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.service_name {
        object.key("serviceName").string(var_4.as_str());
    }
    Ok(())
}

