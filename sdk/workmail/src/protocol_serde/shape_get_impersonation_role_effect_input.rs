// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_impersonation_role_effect_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetImpersonationRoleEffectInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.organization_id {
        object.key("OrganizationId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.impersonation_role_id {
        object.key("ImpersonationRoleId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.target_user {
        object.key("TargetUser").string(var_3.as_str());
    }
    Ok(())
}

