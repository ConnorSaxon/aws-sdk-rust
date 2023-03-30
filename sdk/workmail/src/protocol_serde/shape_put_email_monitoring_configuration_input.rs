// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_email_monitoring_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutEmailMonitoringConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.organization_id {
        object.key("OrganizationId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.role_arn {
        object.key("RoleArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.log_group_arn {
        object.key("LogGroupArn").string(var_3.as_str());
    }
    Ok(())
}

