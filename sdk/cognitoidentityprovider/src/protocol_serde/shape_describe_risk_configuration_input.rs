// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_risk_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeRiskConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.user_pool_id {
        object.key("UserPoolId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_id {
        object.key("ClientId").string(var_2.as_str());
    }
    Ok(())
}

