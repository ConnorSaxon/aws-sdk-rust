// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_stop_build_batch_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StopBuildBatchInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.id {
        object.key("id").string(var_1.as_str());
    }
    Ok(())
}

