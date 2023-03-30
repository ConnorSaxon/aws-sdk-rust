// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_anomaly_monitor_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateAnomalyMonitorInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.monitor_arn {
        object.key("MonitorArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.monitor_name {
        object.key("MonitorName").string(var_2.as_str());
    }
    Ok(())
}

