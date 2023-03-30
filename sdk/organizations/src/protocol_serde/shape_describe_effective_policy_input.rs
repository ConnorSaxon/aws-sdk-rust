// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_effective_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeEffectivePolicyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.policy_type {
        object.key("PolicyType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.target_id {
        object.key("TargetId").string(var_2.as_str());
    }
    Ok(())
}

