// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_tags_for_resource_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListTagsForResourceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.next_marker {
        object.key("NextMarker").string(var_1.as_str());
    }
    if input.limit != 0 {
        object.key("Limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.limit).into()));
    }
    if let Some(var_2) = &input.resource_arn {
        object.key("ResourceARN").string(var_2.as_str());
    }
    Ok(())
}

