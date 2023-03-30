// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_configure_logs_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ConfigureLogsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.egress_access_logs {
        #[allow(unused_mut)]
        let mut object_2 = object.key("egressAccessLogs").start_object();
        crate::protocol_serde::shape_egress_access_logs::ser_egress_access_logs(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

