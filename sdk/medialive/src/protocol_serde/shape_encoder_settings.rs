// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_encoder_settings(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::EncoderSettings) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.audio_descriptions {
        let mut array_2 = object.key("audioDescriptions").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_audio_description::ser_audio_description(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.avail_blanking {
        #[allow(unused_mut)]
        let mut object_6 = object.key("availBlanking").start_object();
        crate::protocol_serde::shape_avail_blanking::ser_avail_blanking(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.avail_configuration {
        #[allow(unused_mut)]
        let mut object_8 = object.key("availConfiguration").start_object();
        crate::protocol_serde::shape_avail_configuration::ser_avail_configuration(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.blackout_slate {
        #[allow(unused_mut)]
        let mut object_10 = object.key("blackoutSlate").start_object();
        crate::protocol_serde::shape_blackout_slate::ser_blackout_slate(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.caption_descriptions {
        let mut array_12 = object.key("captionDescriptions").start_array();
        for item_13 in var_11 {
             {
                #[allow(unused_mut)]
                let mut object_14 = array_12.value().start_object();
                crate::protocol_serde::shape_caption_description::ser_caption_description(&mut object_14, item_13)?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    if let Some(var_15) = &input.feature_activations {
        #[allow(unused_mut)]
        let mut object_16 = object.key("featureActivations").start_object();
        crate::protocol_serde::shape_feature_activations::ser_feature_activations(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.global_configuration {
        #[allow(unused_mut)]
        let mut object_18 = object.key("globalConfiguration").start_object();
        crate::protocol_serde::shape_global_configuration::ser_global_configuration(&mut object_18, var_17)?;
        object_18.finish();
    }
    if let Some(var_19) = &input.motion_graphics_configuration {
        #[allow(unused_mut)]
        let mut object_20 = object.key("motionGraphicsConfiguration").start_object();
        crate::protocol_serde::shape_motion_graphics_configuration::ser_motion_graphics_configuration(&mut object_20, var_19)?;
        object_20.finish();
    }
    if let Some(var_21) = &input.nielsen_configuration {
        #[allow(unused_mut)]
        let mut object_22 = object.key("nielsenConfiguration").start_object();
        crate::protocol_serde::shape_nielsen_configuration::ser_nielsen_configuration(&mut object_22, var_21)?;
        object_22.finish();
    }
    if let Some(var_23) = &input.output_groups {
        let mut array_24 = object.key("outputGroups").start_array();
        for item_25 in var_23 {
             {
                #[allow(unused_mut)]
                let mut object_26 = array_24.value().start_object();
                crate::protocol_serde::shape_output_group::ser_output_group(&mut object_26, item_25)?;
                object_26.finish();
            }
        }
        array_24.finish();
    }
    if let Some(var_27) = &input.timecode_config {
        #[allow(unused_mut)]
        let mut object_28 = object.key("timecodeConfig").start_object();
        crate::protocol_serde::shape_timecode_config::ser_timecode_config(&mut object_28, var_27)?;
        object_28.finish();
    }
    if let Some(var_29) = &input.video_descriptions {
        let mut array_30 = object.key("videoDescriptions").start_array();
        for item_31 in var_29 {
             {
                #[allow(unused_mut)]
                let mut object_32 = array_30.value().start_object();
                crate::protocol_serde::shape_video_description::ser_video_description(&mut object_32, item_31)?;
                object_32.finish();
            }
        }
        array_30.finish();
    }
    Ok(())
}

pub(crate) fn de_encoder_settings<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::EncoderSettings>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::encoder_settings::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "audioDescriptions" => {
                                builder = builder.set_audio_descriptions(
                                    crate::protocol_serde::shape___list_of_audio_description::de___list_of_audio_description(tokens)?
                                );
                            }
                            "availBlanking" => {
                                builder = builder.set_avail_blanking(
                                    crate::protocol_serde::shape_avail_blanking::de_avail_blanking(tokens)?
                                );
                            }
                            "availConfiguration" => {
                                builder = builder.set_avail_configuration(
                                    crate::protocol_serde::shape_avail_configuration::de_avail_configuration(tokens)?
                                );
                            }
                            "blackoutSlate" => {
                                builder = builder.set_blackout_slate(
                                    crate::protocol_serde::shape_blackout_slate::de_blackout_slate(tokens)?
                                );
                            }
                            "captionDescriptions" => {
                                builder = builder.set_caption_descriptions(
                                    crate::protocol_serde::shape___list_of_caption_description::de___list_of_caption_description(tokens)?
                                );
                            }
                            "featureActivations" => {
                                builder = builder.set_feature_activations(
                                    crate::protocol_serde::shape_feature_activations::de_feature_activations(tokens)?
                                );
                            }
                            "globalConfiguration" => {
                                builder = builder.set_global_configuration(
                                    crate::protocol_serde::shape_global_configuration::de_global_configuration(tokens)?
                                );
                            }
                            "motionGraphicsConfiguration" => {
                                builder = builder.set_motion_graphics_configuration(
                                    crate::protocol_serde::shape_motion_graphics_configuration::de_motion_graphics_configuration(tokens)?
                                );
                            }
                            "nielsenConfiguration" => {
                                builder = builder.set_nielsen_configuration(
                                    crate::protocol_serde::shape_nielsen_configuration::de_nielsen_configuration(tokens)?
                                );
                            }
                            "outputGroups" => {
                                builder = builder.set_output_groups(
                                    crate::protocol_serde::shape___list_of_output_group::de___list_of_output_group(tokens)?
                                );
                            }
                            "timecodeConfig" => {
                                builder = builder.set_timecode_config(
                                    crate::protocol_serde::shape_timecode_config::de_timecode_config(tokens)?
                                );
                            }
                            "videoDescriptions" => {
                                builder = builder.set_video_descriptions(
                                    crate::protocol_serde::shape___list_of_video_description::de___list_of_video_description(tokens)?
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

