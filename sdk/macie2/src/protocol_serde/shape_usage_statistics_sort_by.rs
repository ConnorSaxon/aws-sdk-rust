// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_usage_statistics_sort_by(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::UsageStatisticsSortBy) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.key {
        object.key("key").string(var_1.as_str());
    }
    if let Some(var_2) = &input.order_by {
        object.key("orderBy").string(var_2.as_str());
    }
    Ok(())
}

