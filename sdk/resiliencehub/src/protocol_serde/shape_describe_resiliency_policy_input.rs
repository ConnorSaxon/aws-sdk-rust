// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_resiliency_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeResiliencyPolicyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.policy_arn {
        object.key("policyArn").string(var_1.as_str());
    }
    Ok(())
}

