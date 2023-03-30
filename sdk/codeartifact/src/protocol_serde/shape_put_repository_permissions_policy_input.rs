// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_repository_permissions_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutRepositoryPermissionsPolicyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.policy_document {
        object.key("policyDocument").string(var_1.as_str());
    }
    if let Some(var_2) = &input.policy_revision {
        object.key("policyRevision").string(var_2.as_str());
    }
    Ok(())
}

