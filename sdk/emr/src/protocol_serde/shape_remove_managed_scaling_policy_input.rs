// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_remove_managed_scaling_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RemoveManagedScalingPolicyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.cluster_id {
        object.key("ClusterId").string(var_1.as_str());
    }
    Ok(())
}

