// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disassociate_member_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DisassociateMemberInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.account_id {
        object.key("accountId").string(var_1.as_str());
    }
    Ok(())
}

