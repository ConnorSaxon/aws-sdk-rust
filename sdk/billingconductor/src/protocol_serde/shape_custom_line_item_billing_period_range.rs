// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_custom_line_item_billing_period_range(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::CustomLineItemBillingPeriodRange) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.inclusive_start_billing_period {
        object.key("InclusiveStartBillingPeriod").string(var_1.as_str());
    }
    if let Some(var_2) = &input.exclusive_end_billing_period {
        object.key("ExclusiveEndBillingPeriod").string(var_2.as_str());
    }
    Ok(())
}

