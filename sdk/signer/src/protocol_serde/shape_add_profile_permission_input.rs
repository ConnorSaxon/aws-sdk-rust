// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_add_profile_permission_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AddProfilePermissionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.action {
        object.key("action").string(var_1.as_str());
    }
    if let Some(var_2) = &input.principal {
        object.key("principal").string(var_2.as_str());
    }
    if let Some(var_3) = &input.profile_version {
        object.key("profileVersion").string(var_3.as_str());
    }
    if let Some(var_4) = &input.revision_id {
        object.key("revisionId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.statement_id {
        object.key("statementId").string(var_5.as_str());
    }
    Ok(())
}

