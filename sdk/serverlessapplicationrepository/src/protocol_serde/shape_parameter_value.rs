// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_parameter_value(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ParameterValue) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.value {
        object.key("value").string(var_2.as_str());
    }
    Ok(())
}

