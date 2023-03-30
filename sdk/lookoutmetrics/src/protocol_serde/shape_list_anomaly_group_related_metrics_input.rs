// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_anomaly_group_related_metrics_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListAnomalyGroupRelatedMetricsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.anomaly_detector_arn {
        object.key("AnomalyDetectorArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.anomaly_group_id {
        object.key("AnomalyGroupId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_3).into()));
    }
    if let Some(var_4) = &input.next_token {
        object.key("NextToken").string(var_4.as_str());
    }
    if let Some(var_5) = &input.relationship_type_filter {
        object.key("RelationshipTypeFilter").string(var_5.as_str());
    }
    Ok(())
}

