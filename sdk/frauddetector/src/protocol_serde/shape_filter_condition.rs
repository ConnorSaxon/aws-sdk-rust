// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_filter_condition(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::FilterCondition) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.value {
        object.key("value").string(var_1.as_str());
    }
    Ok(())
}

