// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_deployment_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetDeploymentInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.deployment_id {
        object.key("deploymentId").string(var_1.as_str());
    }
    Ok(())
}

