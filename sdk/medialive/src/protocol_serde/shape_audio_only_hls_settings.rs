// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_audio_only_hls_settings(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AudioOnlyHlsSettings) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.audio_group_id {
        object.key("audioGroupId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.audio_only_image {
        #[allow(unused_mut)]
        let mut object_3 = object.key("audioOnlyImage").start_object();
        crate::protocol_serde::shape_input_location::ser_input_location(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.audio_track_type {
        object.key("audioTrackType").string(var_4.as_str());
    }
    if let Some(var_5) = &input.segment_type {
        object.key("segmentType").string(var_5.as_str());
    }
    Ok(())
}

pub(crate) fn de_audio_only_hls_settings<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AudioOnlyHlsSettings>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::audio_only_hls_settings::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "audioGroupId" => {
                                builder = builder.set_audio_group_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "audioOnlyImage" => {
                                builder = builder.set_audio_only_image(
                                    crate::protocol_serde::shape_input_location::de_input_location(tokens)?
                                );
                            }
                            "audioTrackType" => {
                                builder = builder.set_audio_track_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::AudioOnlyHlsTrackType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "segmentType" => {
                                builder = builder.set_segment_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::AudioOnlyHlsSegmentType::from(u.as_ref())
                                        )
                                    ).transpose()?
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

