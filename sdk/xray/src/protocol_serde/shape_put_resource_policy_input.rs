// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_resource_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutResourcePolicyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.bypass_policy_lockout_check {
        object.key("BypassPolicyLockoutCheck").boolean(input.bypass_policy_lockout_check);
    }
    if let Some(var_1) = &input.policy_document {
        object.key("PolicyDocument").string(var_1.as_str());
    }
    if let Some(var_2) = &input.policy_name {
        object.key("PolicyName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.policy_revision_id {
        object.key("PolicyRevisionId").string(var_3.as_str());
    }
    Ok(())
}

