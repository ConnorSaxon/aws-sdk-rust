// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_email_monitoring_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteEmailMonitoringConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.organization_id {
        object.key("OrganizationId").string(var_1.as_str());
    }
    Ok(())
}

