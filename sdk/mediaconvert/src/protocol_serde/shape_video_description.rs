// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_video_description(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::VideoDescription) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.afd_signaling {
        object.key("afdSignaling").string(var_1.as_str());
    }
    if let Some(var_2) = &input.anti_alias {
        object.key("antiAlias").string(var_2.as_str());
    }
    if let Some(var_3) = &input.codec_settings {
        #[allow(unused_mut)]
        let mut object_4 = object.key("codecSettings").start_object();
        crate::protocol_serde::shape_video_codec_settings::ser_video_codec_settings(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.color_metadata {
        object.key("colorMetadata").string(var_5.as_str());
    }
    if let Some(var_6) = &input.crop {
        #[allow(unused_mut)]
        let mut object_7 = object.key("crop").start_object();
        crate::protocol_serde::shape_rectangle::ser_rectangle(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.drop_frame_timecode {
        object.key("dropFrameTimecode").string(var_8.as_str());
    }
    if input.fixed_afd != 0 {
        object.key("fixedAfd").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.fixed_afd).into()));
    }
    if input.height != 0 {
        object.key("height").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.height).into()));
    }
    if let Some(var_9) = &input.position {
        #[allow(unused_mut)]
        let mut object_10 = object.key("position").start_object();
        crate::protocol_serde::shape_rectangle::ser_rectangle(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.respond_to_afd {
        object.key("respondToAfd").string(var_11.as_str());
    }
    if let Some(var_12) = &input.scaling_behavior {
        object.key("scalingBehavior").string(var_12.as_str());
    }
    if input.sharpness != 0 {
        object.key("sharpness").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.sharpness).into()));
    }
    if let Some(var_13) = &input.timecode_insertion {
        object.key("timecodeInsertion").string(var_13.as_str());
    }
    if let Some(var_14) = &input.video_preprocessors {
        #[allow(unused_mut)]
        let mut object_15 = object.key("videoPreprocessors").start_object();
        crate::protocol_serde::shape_video_preprocessor::ser_video_preprocessor(&mut object_15, var_14)?;
        object_15.finish();
    }
    if input.width != 0 {
        object.key("width").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.width).into()));
    }
    Ok(())
}

pub(crate) fn de_video_description<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::VideoDescription>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::video_description::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "afdSignaling" => {
                                builder = builder.set_afd_signaling(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::AfdSignaling::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "antiAlias" => {
                                builder = builder.set_anti_alias(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::AntiAlias::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "codecSettings" => {
                                builder = builder.set_codec_settings(
                                    crate::protocol_serde::shape_video_codec_settings::de_video_codec_settings(tokens)?
                                );
                            }
                            "colorMetadata" => {
                                builder = builder.set_color_metadata(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ColorMetadata::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "crop" => {
                                builder = builder.set_crop(
                                    crate::protocol_serde::shape_rectangle::de_rectangle(tokens)?
                                );
                            }
                            "dropFrameTimecode" => {
                                builder = builder.set_drop_frame_timecode(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::DropFrameTimecode::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "fixedAfd" => {
                                builder = builder.set_fixed_afd(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "height" => {
                                builder = builder.set_height(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "position" => {
                                builder = builder.set_position(
                                    crate::protocol_serde::shape_rectangle::de_rectangle(tokens)?
                                );
                            }
                            "respondToAfd" => {
                                builder = builder.set_respond_to_afd(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::RespondToAfd::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "scalingBehavior" => {
                                builder = builder.set_scaling_behavior(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ScalingBehavior::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "sharpness" => {
                                builder = builder.set_sharpness(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "timecodeInsertion" => {
                                builder = builder.set_timecode_insertion(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::VideoTimecodeInsertion::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "videoPreprocessors" => {
                                builder = builder.set_video_preprocessors(
                                    crate::protocol_serde::shape_video_preprocessor::de_video_preprocessor(tokens)?
                                );
                            }
                            "width" => {
                                builder = builder.set_width(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
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

