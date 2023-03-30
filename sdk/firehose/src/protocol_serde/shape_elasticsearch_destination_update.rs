// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_elasticsearch_destination_update(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ElasticsearchDestinationUpdate) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.role_arn {
        object.key("RoleARN").string(var_1.as_str());
    }
    if let Some(var_2) = &input.domain_arn {
        object.key("DomainARN").string(var_2.as_str());
    }
    if let Some(var_3) = &input.cluster_endpoint {
        object.key("ClusterEndpoint").string(var_3.as_str());
    }
    if let Some(var_4) = &input.index_name {
        object.key("IndexName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.type_name {
        object.key("TypeName").string(var_5.as_str());
    }
    if let Some(var_6) = &input.index_rotation_period {
        object.key("IndexRotationPeriod").string(var_6.as_str());
    }
    if let Some(var_7) = &input.buffering_hints {
        #[allow(unused_mut)]
        let mut object_8 = object.key("BufferingHints").start_object();
        crate::protocol_serde::shape_elasticsearch_buffering_hints::ser_elasticsearch_buffering_hints(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.retry_options {
        #[allow(unused_mut)]
        let mut object_10 = object.key("RetryOptions").start_object();
        crate::protocol_serde::shape_elasticsearch_retry_options::ser_elasticsearch_retry_options(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.s3_update {
        #[allow(unused_mut)]
        let mut object_12 = object.key("S3Update").start_object();
        crate::protocol_serde::shape_s3_destination_update::ser_s3_destination_update(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.processing_configuration {
        #[allow(unused_mut)]
        let mut object_14 = object.key("ProcessingConfiguration").start_object();
        crate::protocol_serde::shape_processing_configuration::ser_processing_configuration(&mut object_14, var_13)?;
        object_14.finish();
    }
    if let Some(var_15) = &input.cloud_watch_logging_options {
        #[allow(unused_mut)]
        let mut object_16 = object.key("CloudWatchLoggingOptions").start_object();
        crate::protocol_serde::shape_cloud_watch_logging_options::ser_cloud_watch_logging_options(&mut object_16, var_15)?;
        object_16.finish();
    }
    Ok(())
}

