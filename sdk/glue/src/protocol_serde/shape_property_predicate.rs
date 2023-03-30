// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_property_predicate(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PropertyPredicate) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.key {
        object.key("Key").string(var_1.as_str());
    }
    if let Some(var_2) = &input.value {
        object.key("Value").string(var_2.as_str());
    }
    if let Some(var_3) = &input.comparator {
        object.key("Comparator").string(var_3.as_str());
    }
    Ok(())
}

