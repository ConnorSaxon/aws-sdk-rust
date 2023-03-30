// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_stream_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeStreamInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.stream_arn {
        object.key("StreamArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.limit {
        object.key("Limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    if let Some(var_3) = &input.exclusive_start_shard_id {
        object.key("ExclusiveStartShardId").string(var_3.as_str());
    }
    Ok(())
}

