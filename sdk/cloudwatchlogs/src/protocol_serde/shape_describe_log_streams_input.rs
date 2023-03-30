// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_log_streams_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeLogStreamsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.log_group_name {
        object.key("logGroupName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.log_group_identifier {
        object.key("logGroupIdentifier").string(var_2.as_str());
    }
    if let Some(var_3) = &input.log_stream_name_prefix {
        object.key("logStreamNamePrefix").string(var_3.as_str());
    }
    if let Some(var_4) = &input.order_by {
        object.key("orderBy").string(var_4.as_str());
    }
    if let Some(var_5) = &input.descending {
        object.key("descending").boolean(*var_5);
    }
    if let Some(var_6) = &input.next_token {
        object.key("nextToken").string(var_6.as_str());
    }
    if let Some(var_7) = &input.limit {
        object.key("limit").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_7).into()));
    }
    Ok(())
}

