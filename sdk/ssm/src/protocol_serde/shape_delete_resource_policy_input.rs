// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_resource_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteResourcePolicyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.resource_arn {
        object.key("ResourceArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.policy_id {
        object.key("PolicyId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.policy_hash {
        object.key("PolicyHash").string(var_3.as_str());
    }
    Ok(())
}

