// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_resource_permission_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutResourcePermissionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.action_type {
        object.key("ActionType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.resource_arn {
        object.key("ResourceArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.source_resource_arn {
        object.key("SourceResourceArn").string(var_3.as_str());
    }
    Ok(())
}

