// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_event_destination_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateEventDestinationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.configuration_set_name {
        object.key("ConfigurationSetName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.event_destination_name {
        object.key("EventDestinationName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.enabled {
        object.key("Enabled").boolean(*var_3);
    }
    if let Some(var_4) = &input.matching_event_types {
        let mut array_5 = object.key("MatchingEventTypes").start_array();
        for item_6 in var_4 {
             {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.cloud_watch_logs_destination {
        #[allow(unused_mut)]
        let mut object_8 = object.key("CloudWatchLogsDestination").start_object();
        crate::protocol_serde::shape_cloud_watch_logs_destination::ser_cloud_watch_logs_destination(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.kinesis_firehose_destination {
        #[allow(unused_mut)]
        let mut object_10 = object.key("KinesisFirehoseDestination").start_object();
        crate::protocol_serde::shape_kinesis_firehose_destination::ser_kinesis_firehose_destination(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.sns_destination {
        #[allow(unused_mut)]
        let mut object_12 = object.key("SnsDestination").start_object();
        crate::protocol_serde::shape_sns_destination::ser_sns_destination(&mut object_12, var_11)?;
        object_12.finish();
    }
    Ok(())
}

