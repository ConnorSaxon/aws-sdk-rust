// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_group_membership_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateGroupMembershipInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.identity_store_id {
        object.key("IdentityStoreId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.group_id {
        object.key("GroupId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.member_id {
        #[allow(unused_mut)]
        let mut object_4 = object.key("MemberId").start_object();
        crate::protocol_serde::shape_member_id::ser_member_id(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

