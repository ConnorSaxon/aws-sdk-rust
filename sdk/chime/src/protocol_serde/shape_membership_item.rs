// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_membership_item(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MembershipItem) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.member_id {
        object.key("MemberId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.role {
        object.key("Role").string(var_2.as_str());
    }
    Ok(())
}

