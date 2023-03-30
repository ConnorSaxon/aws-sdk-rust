// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_cmfc_settings(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::CmfcSettings) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.audio_duration {
        object.key("audioDuration").string(var_1.as_str());
    }
    if let Some(var_2) = &input.audio_group_id {
        object.key("audioGroupId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.audio_rendition_sets {
        object.key("audioRenditionSets").string(var_3.as_str());
    }
    if let Some(var_4) = &input.audio_track_type {
        object.key("audioTrackType").string(var_4.as_str());
    }
    if let Some(var_5) = &input.descriptive_video_service_flag {
        object.key("descriptiveVideoServiceFlag").string(var_5.as_str());
    }
    if let Some(var_6) = &input.i_frame_only_manifest {
        object.key("iFrameOnlyManifest").string(var_6.as_str());
    }
    if let Some(var_7) = &input.klv_metadata {
        object.key("klvMetadata").string(var_7.as_str());
    }
    if let Some(var_8) = &input.manifest_metadata_signaling {
        object.key("manifestMetadataSignaling").string(var_8.as_str());
    }
    if let Some(var_9) = &input.scte35_esam {
        object.key("scte35Esam").string(var_9.as_str());
    }
    if let Some(var_10) = &input.scte35_source {
        object.key("scte35Source").string(var_10.as_str());
    }
    if let Some(var_11) = &input.timed_metadata {
        object.key("timedMetadata").string(var_11.as_str());
    }
    if let Some(var_12) = &input.timed_metadata_box_version {
        object.key("timedMetadataBoxVersion").string(var_12.as_str());
    }
    if let Some(var_13) = &input.timed_metadata_scheme_id_uri {
        object.key("timedMetadataSchemeIdUri").string(var_13.as_str());
    }
    if let Some(var_14) = &input.timed_metadata_value {
        object.key("timedMetadataValue").string(var_14.as_str());
    }
    Ok(())
}

pub(crate) fn de_cmfc_settings<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::CmfcSettings>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::cmfc_settings::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "audioDuration" => {
                                builder = builder.set_audio_duration(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::CmfcAudioDuration::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "audioGroupId" => {
                                builder = builder.set_audio_group_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "audioRenditionSets" => {
                                builder = builder.set_audio_rendition_sets(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "audioTrackType" => {
                                builder = builder.set_audio_track_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::CmfcAudioTrackType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "descriptiveVideoServiceFlag" => {
                                builder = builder.set_descriptive_video_service_flag(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::CmfcDescriptiveVideoServiceFlag::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "iFrameOnlyManifest" => {
                                builder = builder.set_i_frame_only_manifest(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::CmfcIFrameOnlyManifest::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "klvMetadata" => {
                                builder = builder.set_klv_metadata(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::CmfcKlvMetadata::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "manifestMetadataSignaling" => {
                                builder = builder.set_manifest_metadata_signaling(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::CmfcManifestMetadataSignaling::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "scte35Esam" => {
                                builder = builder.set_scte35_esam(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::CmfcScte35Esam::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "scte35Source" => {
                                builder = builder.set_scte35_source(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::CmfcScte35Source::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "timedMetadata" => {
                                builder = builder.set_timed_metadata(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::CmfcTimedMetadata::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "timedMetadataBoxVersion" => {
                                builder = builder.set_timed_metadata_box_version(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::CmfcTimedMetadataBoxVersion::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "timedMetadataSchemeIdUri" => {
                                builder = builder.set_timed_metadata_scheme_id_uri(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "timedMetadataValue" => {
                                builder = builder.set_timed_metadata_value(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
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

