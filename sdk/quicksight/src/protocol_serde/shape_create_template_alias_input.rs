// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_template_alias_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateTemplateAliasInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.template_version_number {
        object.key("TemplateVersionNumber").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_1).into()));
    }
    Ok(())
}

