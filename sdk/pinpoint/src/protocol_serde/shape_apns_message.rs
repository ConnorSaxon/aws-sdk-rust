// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_apns_message(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ApnsMessage) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.apns_push_type {
        object.key("APNSPushType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.action {
        object.key("Action").string(var_2.as_str());
    }
    if input.badge != 0 {
        object.key("Badge").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.badge).into()));
    }
    if let Some(var_3) = &input.body {
        object.key("Body").string(var_3.as_str());
    }
    if let Some(var_4) = &input.category {
        object.key("Category").string(var_4.as_str());
    }
    if let Some(var_5) = &input.collapse_id {
        object.key("CollapseId").string(var_5.as_str());
    }
    if let Some(var_6) = &input.data {
        #[allow(unused_mut)]
        let mut object_7 = object.key("Data").start_object();
        for (key_8, value_9) in var_6 {
             {
                object_7.key(key_8.as_str()).string(value_9.as_str());
            }
        }
        object_7.finish();
    }
    if let Some(var_10) = &input.media_url {
        object.key("MediaUrl").string(var_10.as_str());
    }
    if let Some(var_11) = &input.preferred_authentication_method {
        object.key("PreferredAuthenticationMethod").string(var_11.as_str());
    }
    if let Some(var_12) = &input.priority {
        object.key("Priority").string(var_12.as_str());
    }
    if let Some(var_13) = &input.raw_content {
        object.key("RawContent").string(var_13.as_str());
    }
    if input.silent_push {
        object.key("SilentPush").boolean(input.silent_push);
    }
    if let Some(var_14) = &input.sound {
        object.key("Sound").string(var_14.as_str());
    }
    if let Some(var_15) = &input.substitutions {
        #[allow(unused_mut)]
        let mut object_16 = object.key("Substitutions").start_object();
        for (key_17, value_18) in var_15 {
             {
                let mut array_19 = object_16.key(key_17.as_str()).start_array();
                for item_20 in value_18 {
                     {
                        array_19.value().string(item_20.as_str());
                    }
                }
                array_19.finish();
            }
        }
        object_16.finish();
    }
    if let Some(var_21) = &input.thread_id {
        object.key("ThreadId").string(var_21.as_str());
    }
    if input.time_to_live != 0 {
        object.key("TimeToLive").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.time_to_live).into()));
    }
    if let Some(var_22) = &input.title {
        object.key("Title").string(var_22.as_str());
    }
    if let Some(var_23) = &input.url {
        object.key("Url").string(var_23.as_str());
    }
    Ok(())
}

