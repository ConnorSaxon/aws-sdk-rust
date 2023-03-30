// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_metric_monitor_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MetricMonitorConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.metric_definition {
        #[allow(unused_mut)]
        let mut object_2 = object.key("metricDefinition").start_object();
        crate::protocol_serde::shape_metric_definition_config::ser_metric_definition_config(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

