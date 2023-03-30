// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_grant_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetGrantInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.grant_arn {
        object.key("GrantArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.version {
        object.key("Version").string(var_2.as_str());
    }
    Ok(())
}

