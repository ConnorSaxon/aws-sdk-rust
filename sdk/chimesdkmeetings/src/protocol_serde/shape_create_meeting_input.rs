// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_meeting_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateMeetingInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.external_meeting_id {
        object.key("ExternalMeetingId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.media_region {
        object.key("MediaRegion").string(var_3.as_str());
    }
    if let Some(var_4) = &input.meeting_features {
        #[allow(unused_mut)]
        let mut object_5 = object.key("MeetingFeatures").start_object();
        crate::protocol_serde::shape_meeting_features_configuration::ser_meeting_features_configuration(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.meeting_host_id {
        object.key("MeetingHostId").string(var_6.as_str());
    }
    if let Some(var_7) = &input.notifications_configuration {
        #[allow(unused_mut)]
        let mut object_8 = object.key("NotificationsConfiguration").start_object();
        crate::protocol_serde::shape_notifications_configuration::ser_notifications_configuration(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.primary_meeting_id {
        object.key("PrimaryMeetingId").string(var_9.as_str());
    }
    if let Some(var_10) = &input.tags {
        let mut array_11 = object.key("Tags").start_array();
        for item_12 in var_10 {
             {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    if let Some(var_14) = &input.tenant_ids {
        let mut array_15 = object.key("TenantIds").start_array();
        for item_16 in var_14 {
             {
                array_15.value().string(item_16.as_str());
            }
        }
        array_15.finish();
    }
    Ok(())
}

