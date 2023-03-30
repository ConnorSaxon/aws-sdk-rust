// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_repository_policy_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetRepositoryPolicyInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.registry_id {
        object.key("registryId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.repository_name {
        object.key("repositoryName").string(var_2.as_str());
    }
    Ok(())
}

