// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_image_scanning_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutImageScanningConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.registry_id {
        object.key("registryId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.repository_name {
        object.key("repositoryName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.image_scanning_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("imageScanningConfiguration").start_object();
        crate::protocol_serde::shape_image_scanning_configuration::ser_image_scanning_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

