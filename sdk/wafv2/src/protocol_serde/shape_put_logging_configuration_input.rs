// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_logging_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutLoggingConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.logging_configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("LoggingConfiguration").start_object();
        crate::protocol_serde::shape_logging_configuration::ser_logging_configuration(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

