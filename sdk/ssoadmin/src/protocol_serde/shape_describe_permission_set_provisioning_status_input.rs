// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_permission_set_provisioning_status_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribePermissionSetProvisioningStatusInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.instance_arn {
        object.key("InstanceArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.provision_permission_set_request_id {
        object.key("ProvisionPermissionSetRequestId").string(var_2.as_str());
    }
    Ok(())
}

