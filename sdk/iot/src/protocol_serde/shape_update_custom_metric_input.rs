// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_custom_metric_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateCustomMetricInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.display_name {
        object.key("displayName").string(var_1.as_str());
    }
    Ok(())
}

