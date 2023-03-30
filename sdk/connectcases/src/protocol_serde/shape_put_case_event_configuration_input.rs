// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_case_event_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutCaseEventConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.event_bridge {
        #[allow(unused_mut)]
        let mut object_2 = object.key("eventBridge").start_object();
        crate::protocol_serde::shape_event_bridge_configuration::ser_event_bridge_configuration(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

