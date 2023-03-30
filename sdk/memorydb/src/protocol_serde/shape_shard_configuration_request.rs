// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_shard_configuration_request(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ShardConfigurationRequest) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.shard_count != 0 {
        object.key("ShardCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.shard_count).into()));
    }
    Ok(())
}

