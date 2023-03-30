// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_attribute_operation(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AttributeOperation) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.attribute_path {
        object.key("AttributePath").string(var_1.as_str());
    }
    if let Some(var_2) = &input.attribute_value {
        object.key("AttributeValue").document(var_2);
    }
    Ok(())
}

