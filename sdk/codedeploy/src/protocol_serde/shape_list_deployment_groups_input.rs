// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_deployment_groups_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListDeploymentGroupsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.application_name {
        object.key("applicationName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.next_token {
        object.key("nextToken").string(var_2.as_str());
    }
    Ok(())
}

