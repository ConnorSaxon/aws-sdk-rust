// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_queue_outbound_caller_config_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateQueueOutboundCallerConfigInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.outbound_caller_config {
        #[allow(unused_mut)]
        let mut object_2 = object.key("OutboundCallerConfig").start_object();
        crate::protocol_serde::shape_outbound_caller_config::ser_outbound_caller_config(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

