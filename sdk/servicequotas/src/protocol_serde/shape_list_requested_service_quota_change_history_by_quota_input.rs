// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_requested_service_quota_change_history_by_quota_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListRequestedServiceQuotaChangeHistoryByQuotaInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.service_code {
        object.key("ServiceCode").string(var_1.as_str());
    }
    if let Some(var_2) = &input.quota_code {
        object.key("QuotaCode").string(var_2.as_str());
    }
    if let Some(var_3) = &input.status {
        object.key("Status").string(var_3.as_str());
    }
    if let Some(var_4) = &input.next_token {
        object.key("NextToken").string(var_4.as_str());
    }
    if let Some(var_5) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_5).into()));
    }
    Ok(())
}

