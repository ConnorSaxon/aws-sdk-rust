// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_theme_alias_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateThemeAliasInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.theme_version_number {
        object.key("ThemeVersionNumber").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_1).into()));
    }
    Ok(())
}

