// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_block_public_access_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutBlockPublicAccessConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.block_public_access_configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("BlockPublicAccessConfiguration").start_object();
        crate::protocol_serde::shape_block_public_access_configuration::ser_block_public_access_configuration(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

