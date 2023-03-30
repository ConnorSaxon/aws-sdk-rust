// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_ip_set_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteIpSetInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.scope {
        object.key("Scope").string(var_2.as_str());
    }
    if let Some(var_3) = &input.id {
        object.key("Id").string(var_3.as_str());
    }
    if let Some(var_4) = &input.lock_token {
        object.key("LockToken").string(var_4.as_str());
    }
    Ok(())
}

