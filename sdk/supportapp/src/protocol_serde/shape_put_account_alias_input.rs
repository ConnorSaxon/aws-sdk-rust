// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_account_alias_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutAccountAliasInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.account_alias {
        object.key("accountAlias").string(var_1.as_str());
    }
    Ok(())
}

