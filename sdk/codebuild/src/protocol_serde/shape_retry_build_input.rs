// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_retry_build_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RetryBuildInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.id {
        object.key("id").string(var_1.as_str());
    }
    if let Some(var_2) = &input.idempotency_token {
        object.key("idempotencyToken").string(var_2.as_str());
    }
    Ok(())
}

