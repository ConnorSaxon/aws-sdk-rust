// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_schedule_action_settings<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ScheduleActionSettings>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::schedule_action_settings::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "hlsId3SegmentTaggingSettings" => {
                                builder = builder.set_hls_id3_segment_tagging_settings(
                                    crate::protocol_serde::shape_hls_id3_segment_tagging_schedule_action_settings::de_hls_id3_segment_tagging_schedule_action_settings(tokens)?
                                );
                            }
                            "hlsTimedMetadataSettings" => {
                                builder = builder.set_hls_timed_metadata_settings(
                                    crate::protocol_serde::shape_hls_timed_metadata_schedule_action_settings::de_hls_timed_metadata_schedule_action_settings(tokens)?
                                );
                            }
                            "inputPrepareSettings" => {
                                builder = builder.set_input_prepare_settings(
                                    crate::protocol_serde::shape_input_prepare_schedule_action_settings::de_input_prepare_schedule_action_settings(tokens)?
                                );
                            }
                            "inputSwitchSettings" => {
                                builder = builder.set_input_switch_settings(
                                    crate::protocol_serde::shape_input_switch_schedule_action_settings::de_input_switch_schedule_action_settings(tokens)?
                                );
                            }
                            "motionGraphicsImageActivateSettings" => {
                                builder = builder.set_motion_graphics_image_activate_settings(
                                    crate::protocol_serde::shape_motion_graphics_activate_schedule_action_settings::de_motion_graphics_activate_schedule_action_settings(tokens)?
                                );
                            }
                            "motionGraphicsImageDeactivateSettings" => {
                                builder = builder.set_motion_graphics_image_deactivate_settings(
                                    crate::protocol_serde::shape_motion_graphics_deactivate_schedule_action_settings::de_motion_graphics_deactivate_schedule_action_settings(tokens)?
                                );
                            }
                            "pauseStateSettings" => {
                                builder = builder.set_pause_state_settings(
                                    crate::protocol_serde::shape_pause_state_schedule_action_settings::de_pause_state_schedule_action_settings(tokens)?
                                );
                            }
                            "scte35InputSettings" => {
                                builder = builder.set_scte35_input_settings(
                                    crate::protocol_serde::shape_scte35_input_schedule_action_settings::de_scte35_input_schedule_action_settings(tokens)?
                                );
                            }
                            "scte35ReturnToNetworkSettings" => {
                                builder = builder.set_scte35_return_to_network_settings(
                                    crate::protocol_serde::shape_scte35_return_to_network_schedule_action_settings::de_scte35_return_to_network_schedule_action_settings(tokens)?
                                );
                            }
                            "scte35SpliceInsertSettings" => {
                                builder = builder.set_scte35_splice_insert_settings(
                                    crate::protocol_serde::shape_scte35_splice_insert_schedule_action_settings::de_scte35_splice_insert_schedule_action_settings(tokens)?
                                );
                            }
                            "scte35TimeSignalSettings" => {
                                builder = builder.set_scte35_time_signal_settings(
                                    crate::protocol_serde::shape_scte35_time_signal_schedule_action_settings::de_scte35_time_signal_schedule_action_settings(tokens)?
                                );
                            }
                            "staticImageActivateSettings" => {
                                builder = builder.set_static_image_activate_settings(
                                    crate::protocol_serde::shape_static_image_activate_schedule_action_settings::de_static_image_activate_schedule_action_settings(tokens)?
                                );
                            }
                            "staticImageDeactivateSettings" => {
                                builder = builder.set_static_image_deactivate_settings(
                                    crate::protocol_serde::shape_static_image_deactivate_schedule_action_settings::de_static_image_deactivate_schedule_action_settings(tokens)?
                                );
                            }
                            _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                        }
                    }
                    other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
            Ok(Some(builder.build()))
        }
        _ => {
            Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
        }
    }
}

pub fn ser_schedule_action_settings(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ScheduleActionSettings) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.hls_id3_segment_tagging_settings {
        #[allow(unused_mut)]
        let mut object_2 = object.key("hlsId3SegmentTaggingSettings").start_object();
        crate::protocol_serde::shape_hls_id3_segment_tagging_schedule_action_settings::ser_hls_id3_segment_tagging_schedule_action_settings(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.hls_timed_metadata_settings {
        #[allow(unused_mut)]
        let mut object_4 = object.key("hlsTimedMetadataSettings").start_object();
        crate::protocol_serde::shape_hls_timed_metadata_schedule_action_settings::ser_hls_timed_metadata_schedule_action_settings(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.input_prepare_settings {
        #[allow(unused_mut)]
        let mut object_6 = object.key("inputPrepareSettings").start_object();
        crate::protocol_serde::shape_input_prepare_schedule_action_settings::ser_input_prepare_schedule_action_settings(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.input_switch_settings {
        #[allow(unused_mut)]
        let mut object_8 = object.key("inputSwitchSettings").start_object();
        crate::protocol_serde::shape_input_switch_schedule_action_settings::ser_input_switch_schedule_action_settings(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.motion_graphics_image_activate_settings {
        #[allow(unused_mut)]
        let mut object_10 = object.key("motionGraphicsImageActivateSettings").start_object();
        crate::protocol_serde::shape_motion_graphics_activate_schedule_action_settings::ser_motion_graphics_activate_schedule_action_settings(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.motion_graphics_image_deactivate_settings {
        #[allow(unused_mut)]
        let mut object_12 = object.key("motionGraphicsImageDeactivateSettings").start_object();
        crate::protocol_serde::shape_motion_graphics_deactivate_schedule_action_settings::ser_motion_graphics_deactivate_schedule_action_settings(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.pause_state_settings {
        #[allow(unused_mut)]
        let mut object_14 = object.key("pauseStateSettings").start_object();
        crate::protocol_serde::shape_pause_state_schedule_action_settings::ser_pause_state_schedule_action_settings(&mut object_14, var_13)?;
        object_14.finish();
    }
    if let Some(var_15) = &input.scte35_input_settings {
        #[allow(unused_mut)]
        let mut object_16 = object.key("scte35InputSettings").start_object();
        crate::protocol_serde::shape_scte35_input_schedule_action_settings::ser_scte35_input_schedule_action_settings(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.scte35_return_to_network_settings {
        #[allow(unused_mut)]
        let mut object_18 = object.key("scte35ReturnToNetworkSettings").start_object();
        crate::protocol_serde::shape_scte35_return_to_network_schedule_action_settings::ser_scte35_return_to_network_schedule_action_settings(&mut object_18, var_17)?;
        object_18.finish();
    }
    if let Some(var_19) = &input.scte35_splice_insert_settings {
        #[allow(unused_mut)]
        let mut object_20 = object.key("scte35SpliceInsertSettings").start_object();
        crate::protocol_serde::shape_scte35_splice_insert_schedule_action_settings::ser_scte35_splice_insert_schedule_action_settings(&mut object_20, var_19)?;
        object_20.finish();
    }
    if let Some(var_21) = &input.scte35_time_signal_settings {
        #[allow(unused_mut)]
        let mut object_22 = object.key("scte35TimeSignalSettings").start_object();
        crate::protocol_serde::shape_scte35_time_signal_schedule_action_settings::ser_scte35_time_signal_schedule_action_settings(&mut object_22, var_21)?;
        object_22.finish();
    }
    if let Some(var_23) = &input.static_image_activate_settings {
        #[allow(unused_mut)]
        let mut object_24 = object.key("staticImageActivateSettings").start_object();
        crate::protocol_serde::shape_static_image_activate_schedule_action_settings::ser_static_image_activate_schedule_action_settings(&mut object_24, var_23)?;
        object_24.finish();
    }
    if let Some(var_25) = &input.static_image_deactivate_settings {
        #[allow(unused_mut)]
        let mut object_26 = object.key("staticImageDeactivateSettings").start_object();
        crate::protocol_serde::shape_static_image_deactivate_schedule_action_settings::ser_static_image_deactivate_schedule_action_settings(&mut object_26, var_25)?;
        object_26.finish();
    }
    Ok(())
}

