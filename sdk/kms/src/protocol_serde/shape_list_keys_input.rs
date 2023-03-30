// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_keys_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListKeysInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.limit {
        object.key("Limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_1).into()));
    }
    if let Some(var_2) = &input.marker {
        object.key("Marker").string(var_2.as_str());
    }
    Ok(())
}

