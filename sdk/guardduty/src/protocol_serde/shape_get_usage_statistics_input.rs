// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_usage_statistics_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetUsageStatisticsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.max_results != 0 {
        object.key("maxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_results).into()));
    }
    if let Some(var_1) = &input.next_token {
        object.key("nextToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.unit {
        object.key("unit").string(var_2.as_str());
    }
    if let Some(var_3) = &input.usage_criteria {
        #[allow(unused_mut)]
        let mut object_4 = object.key("usageCriteria").start_object();
        crate::protocol_serde::shape_usage_criteria::ser_usage_criteria(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.usage_statistic_type {
        object.key("usageStatisticsType").string(var_5.as_str());
    }
    Ok(())
}

