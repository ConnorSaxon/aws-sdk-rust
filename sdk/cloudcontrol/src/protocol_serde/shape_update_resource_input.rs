// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.type_name {
        object.key("TypeName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.type_version_id {
        object.key("TypeVersionId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.role_arn {
        object.key("RoleArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.client_token {
        object.key("ClientToken").string(var_4.as_str());
    }
    if let Some(var_5) = &input.identifier {
        object.key("Identifier").string(var_5.as_str());
    }
    if let Some(var_6) = &input.patch_document {
        object.key("PatchDocument").string(var_6.as_str());
    }
    Ok(())
}

