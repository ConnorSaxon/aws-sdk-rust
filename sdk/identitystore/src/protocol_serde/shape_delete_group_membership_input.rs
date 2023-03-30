// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_group_membership_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteGroupMembershipInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.identity_store_id {
        object.key("IdentityStoreId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.membership_id {
        object.key("MembershipId").string(var_2.as_str());
    }
    Ok(())
}

