// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_splunk_destination_update(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SplunkDestinationUpdate) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.hec_endpoint {
        object.key("HECEndpoint").string(var_1.as_str());
    }
    if let Some(var_2) = &input.hec_endpoint_type {
        object.key("HECEndpointType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.hec_token {
        object.key("HECToken").string(var_3.as_str());
    }
    if let Some(var_4) = &input.hec_acknowledgment_timeout_in_seconds {
        object.key("HECAcknowledgmentTimeoutInSeconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_4).into()));
    }
    if let Some(var_5) = &input.retry_options {
        #[allow(unused_mut)]
        let mut object_6 = object.key("RetryOptions").start_object();
        crate::protocol_serde::shape_splunk_retry_options::ser_splunk_retry_options(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.s3_backup_mode {
        object.key("S3BackupMode").string(var_7.as_str());
    }
    if let Some(var_8) = &input.s3_update {
        #[allow(unused_mut)]
        let mut object_9 = object.key("S3Update").start_object();
        crate::protocol_serde::shape_s3_destination_update::ser_s3_destination_update(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.processing_configuration {
        #[allow(unused_mut)]
        let mut object_11 = object.key("ProcessingConfiguration").start_object();
        crate::protocol_serde::shape_processing_configuration::ser_processing_configuration(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.cloud_watch_logging_options {
        #[allow(unused_mut)]
        let mut object_13 = object.key("CloudWatchLoggingOptions").start_object();
        crate::protocol_serde::shape_cloud_watch_logging_options::ser_cloud_watch_logging_options(&mut object_13, var_12)?;
        object_13.finish();
    }
    Ok(())
}

