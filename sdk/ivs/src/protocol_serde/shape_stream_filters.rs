// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_stream_filters(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::StreamFilters) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.health {
        object.key("health").string(var_1.as_str());
    }
    Ok(())
}

