// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_stack_provisioning_parameters_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeStackProvisioningParametersInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.stack_id {
        object.key("StackId").string(var_1.as_str());
    }
    Ok(())
}

