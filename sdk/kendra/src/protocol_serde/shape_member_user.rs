// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_member_user(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MemberUser) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.user_id {
        object.key("UserId").string(var_1.as_str());
    }
    Ok(())
}

