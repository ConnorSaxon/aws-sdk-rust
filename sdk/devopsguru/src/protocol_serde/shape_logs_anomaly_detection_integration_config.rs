// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_logs_anomaly_detection_integration_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::LogsAnomalyDetectionIntegrationConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.opt_in_status {
        object.key("OptInStatus").string(var_1.as_str());
    }
    Ok(())
}

