// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_budget_performance_history_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeBudgetPerformanceHistoryInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.account_id {
        object.key("AccountId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.budget_name {
        object.key("BudgetName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.time_period {
        #[allow(unused_mut)]
        let mut object_4 = object.key("TimePeriod").start_object();
        crate::protocol_serde::shape_time_period::ser_time_period(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_5).into()));
    }
    if let Some(var_6) = &input.next_token {
        object.key("NextToken").string(var_6.as_str());
    }
    Ok(())
}

