// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_image_version_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteImageVersionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.image_name {
        object.key("ImageName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.version {
        object.key("Version").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    if let Some(var_3) = &input.alias {
        object.key("Alias").string(var_3.as_str());
    }
    Ok(())
}

