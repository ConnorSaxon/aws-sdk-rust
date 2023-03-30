// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_stack_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateStackInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.display_name {
        object.key("DisplayName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.storage_connectors {
        let mut array_5 = object.key("StorageConnectors").start_array();
        for item_6 in var_4 {
             {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_storage_connector::ser_storage_connector(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.redirect_url {
        object.key("RedirectURL").string(var_8.as_str());
    }
    if let Some(var_9) = &input.feedback_url {
        object.key("FeedbackURL").string(var_9.as_str());
    }
    if let Some(var_10) = &input.user_settings {
        let mut array_11 = object.key("UserSettings").start_array();
        for item_12 in var_10 {
             {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_user_setting::ser_user_setting(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    if let Some(var_14) = &input.application_settings {
        #[allow(unused_mut)]
        let mut object_15 = object.key("ApplicationSettings").start_object();
        crate::protocol_serde::shape_application_settings::ser_application_settings(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.tags {
        #[allow(unused_mut)]
        let mut object_17 = object.key("Tags").start_object();
        for (key_18, value_19) in var_16 {
             {
                object_17.key(key_18.as_str()).string(value_19.as_str());
            }
        }
        object_17.finish();
    }
    if let Some(var_20) = &input.access_endpoints {
        let mut array_21 = object.key("AccessEndpoints").start_array();
        for item_22 in var_20 {
             {
                #[allow(unused_mut)]
                let mut object_23 = array_21.value().start_object();
                crate::protocol_serde::shape_access_endpoint::ser_access_endpoint(&mut object_23, item_22)?;
                object_23.finish();
            }
        }
        array_21.finish();
    }
    if let Some(var_24) = &input.embed_host_domains {
        let mut array_25 = object.key("EmbedHostDomains").start_array();
        for item_26 in var_24 {
             {
                array_25.value().string(item_26.as_str());
            }
        }
        array_25.finish();
    }
    if let Some(var_27) = &input.streaming_experience_settings {
        #[allow(unused_mut)]
        let mut object_28 = object.key("StreamingExperienceSettings").start_object();
        crate::protocol_serde::shape_streaming_experience_settings::ser_streaming_experience_settings(&mut object_28, var_27)?;
        object_28.finish();
    }
    Ok(())
}

