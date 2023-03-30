// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_anomaly_monitors_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetAnomalyMonitorsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.monitor_arn_list {
        let mut array_2 = object.key("MonitorArnList").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.next_page_token {
        object.key("NextPageToken").string(var_4.as_str());
    }
    if let Some(var_5) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_5).into()));
    }
    Ok(())
}

