// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_verify_user_attribute_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::VerifyUserAttributeInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.access_token {
        object.key("AccessToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.attribute_name {
        object.key("AttributeName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.code {
        object.key("Code").string(var_3.as_str());
    }
    Ok(())
}

