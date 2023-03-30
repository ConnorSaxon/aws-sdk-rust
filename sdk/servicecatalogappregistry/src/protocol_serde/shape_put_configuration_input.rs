// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("configuration").start_object();
        crate::protocol_serde::shape_app_registry_configuration::ser_app_registry_configuration(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

