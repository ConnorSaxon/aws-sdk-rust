// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_policy_version_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreatePolicyVersionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.policy_document {
        object.key("policyDocument").string(var_1.as_str());
    }
    Ok(())
}

