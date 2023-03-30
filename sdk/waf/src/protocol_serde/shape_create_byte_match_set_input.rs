// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_byte_match_set_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateByteMatchSetInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.change_token {
        object.key("ChangeToken").string(var_2.as_str());
    }
    Ok(())
}

