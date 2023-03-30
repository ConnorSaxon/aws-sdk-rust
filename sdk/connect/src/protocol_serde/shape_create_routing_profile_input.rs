// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_routing_profile_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateRoutingProfileInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.default_outbound_queue_id {
        object.key("DefaultOutboundQueueId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.media_concurrencies {
        let mut array_4 = object.key("MediaConcurrencies").start_array();
        for item_5 in var_3 {
             {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_media_concurrency::ser_media_concurrency(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.name {
        object.key("Name").string(var_7.as_str());
    }
    if let Some(var_8) = &input.queue_configs {
        let mut array_9 = object.key("QueueConfigs").start_array();
        for item_10 in var_8 {
             {
                #[allow(unused_mut)]
                let mut object_11 = array_9.value().start_object();
                crate::protocol_serde::shape_routing_profile_queue_config::ser_routing_profile_queue_config(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    if let Some(var_12) = &input.tags {
        #[allow(unused_mut)]
        let mut object_13 = object.key("Tags").start_object();
        for (key_14, value_15) in var_12 {
             {
                object_13.key(key_14.as_str()).string(value_15.as_str());
            }
        }
        object_13.finish();
    }
    Ok(())
}

