// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_audit_stream_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateAuditStreamConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.audit_stream_arn {
        object.key("AuditStreamArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.fleet_arn {
        object.key("FleetArn").string(var_2.as_str());
    }
    Ok(())
}

