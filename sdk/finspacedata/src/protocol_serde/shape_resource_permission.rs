// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_resource_permission(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ResourcePermission) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.permission {
        object.key("permission").string(var_1.as_str());
    }
    Ok(())
}

