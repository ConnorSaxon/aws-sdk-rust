// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_hls_group_settings(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::HlsGroupSettings) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.ad_markers {
        let mut array_2 = object.key("adMarkers").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.additional_manifests {
        let mut array_5 = object.key("additionalManifests").start_array();
        for item_6 in var_4 {
             {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_hls_additional_manifest::ser_hls_additional_manifest(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.audio_only_header {
        object.key("audioOnlyHeader").string(var_8.as_str());
    }
    if let Some(var_9) = &input.base_url {
        object.key("baseUrl").string(var_9.as_str());
    }
    if let Some(var_10) = &input.caption_language_mappings {
        let mut array_11 = object.key("captionLanguageMappings").start_array();
        for item_12 in var_10 {
             {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_hls_caption_language_mapping::ser_hls_caption_language_mapping(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    if let Some(var_14) = &input.caption_language_setting {
        object.key("captionLanguageSetting").string(var_14.as_str());
    }
    if let Some(var_15) = &input.caption_segment_length_control {
        object.key("captionSegmentLengthControl").string(var_15.as_str());
    }
    if let Some(var_16) = &input.client_cache {
        object.key("clientCache").string(var_16.as_str());
    }
    if let Some(var_17) = &input.codec_specification {
        object.key("codecSpecification").string(var_17.as_str());
    }
    if let Some(var_18) = &input.destination {
        object.key("destination").string(var_18.as_str());
    }
    if let Some(var_19) = &input.destination_settings {
        #[allow(unused_mut)]
        let mut object_20 = object.key("destinationSettings").start_object();
        crate::protocol_serde::shape_destination_settings::ser_destination_settings(&mut object_20, var_19)?;
        object_20.finish();
    }
    if let Some(var_21) = &input.directory_structure {
        object.key("directoryStructure").string(var_21.as_str());
    }
    if let Some(var_22) = &input.encryption {
        #[allow(unused_mut)]
        let mut object_23 = object.key("encryption").start_object();
        crate::protocol_serde::shape_hls_encryption_settings::ser_hls_encryption_settings(&mut object_23, var_22)?;
        object_23.finish();
    }
    if let Some(var_24) = &input.image_based_trick_play {
        object.key("imageBasedTrickPlay").string(var_24.as_str());
    }
    if let Some(var_25) = &input.image_based_trick_play_settings {
        #[allow(unused_mut)]
        let mut object_26 = object.key("imageBasedTrickPlaySettings").start_object();
        crate::protocol_serde::shape_hls_image_based_trick_play_settings::ser_hls_image_based_trick_play_settings(&mut object_26, var_25)?;
        object_26.finish();
    }
    if let Some(var_27) = &input.manifest_compression {
        object.key("manifestCompression").string(var_27.as_str());
    }
    if let Some(var_28) = &input.manifest_duration_format {
        object.key("manifestDurationFormat").string(var_28.as_str());
    }
    if input.min_final_segment_length != 0.0 {
        object.key("minFinalSegmentLength").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((input.min_final_segment_length).into()));
    }
    if input.min_segment_length != 0 {
        object.key("minSegmentLength").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.min_segment_length).into()));
    }
    if let Some(var_29) = &input.output_selection {
        object.key("outputSelection").string(var_29.as_str());
    }
    if let Some(var_30) = &input.program_date_time {
        object.key("programDateTime").string(var_30.as_str());
    }
    if input.program_date_time_period != 0 {
        object.key("programDateTimePeriod").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.program_date_time_period).into()));
    }
    if let Some(var_31) = &input.segment_control {
        object.key("segmentControl").string(var_31.as_str());
    }
    if input.segment_length != 0 {
        object.key("segmentLength").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.segment_length).into()));
    }
    if let Some(var_32) = &input.segment_length_control {
        object.key("segmentLengthControl").string(var_32.as_str());
    }
    if input.segments_per_subdirectory != 0 {
        object.key("segmentsPerSubdirectory").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.segments_per_subdirectory).into()));
    }
    if let Some(var_33) = &input.stream_inf_resolution {
        object.key("streamInfResolution").string(var_33.as_str());
    }
    if let Some(var_34) = &input.target_duration_compatibility_mode {
        object.key("targetDurationCompatibilityMode").string(var_34.as_str());
    }
    if let Some(var_35) = &input.timed_metadata_id3_frame {
        object.key("timedMetadataId3Frame").string(var_35.as_str());
    }
    if input.timed_metadata_id3_period != 0 {
        object.key("timedMetadataId3Period").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.timed_metadata_id3_period).into()));
    }
    if input.timestamp_delta_milliseconds != 0 {
        object.key("timestampDeltaMilliseconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.timestamp_delta_milliseconds).into()));
    }
    Ok(())
}

pub(crate) fn de_hls_group_settings<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::HlsGroupSettings>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::hls_group_settings::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "adMarkers" => {
                                builder = builder.set_ad_markers(
                                    crate::protocol_serde::shape___list_of_hls_ad_markers::de___list_of_hls_ad_markers(tokens)?
                                );
                            }
                            "additionalManifests" => {
                                builder = builder.set_additional_manifests(
                                    crate::protocol_serde::shape___list_of_hls_additional_manifest::de___list_of_hls_additional_manifest(tokens)?
                                );
                            }
                            "audioOnlyHeader" => {
                                builder = builder.set_audio_only_header(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsAudioOnlyHeader::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "baseUrl" => {
                                builder = builder.set_base_url(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "captionLanguageMappings" => {
                                builder = builder.set_caption_language_mappings(
                                    crate::protocol_serde::shape___list_of_hls_caption_language_mapping::de___list_of_hls_caption_language_mapping(tokens)?
                                );
                            }
                            "captionLanguageSetting" => {
                                builder = builder.set_caption_language_setting(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsCaptionLanguageSetting::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "captionSegmentLengthControl" => {
                                builder = builder.set_caption_segment_length_control(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsCaptionSegmentLengthControl::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "clientCache" => {
                                builder = builder.set_client_cache(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsClientCache::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "codecSpecification" => {
                                builder = builder.set_codec_specification(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsCodecSpecification::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "destination" => {
                                builder = builder.set_destination(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "destinationSettings" => {
                                builder = builder.set_destination_settings(
                                    crate::protocol_serde::shape_destination_settings::de_destination_settings(tokens)?
                                );
                            }
                            "directoryStructure" => {
                                builder = builder.set_directory_structure(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsDirectoryStructure::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "encryption" => {
                                builder = builder.set_encryption(
                                    crate::protocol_serde::shape_hls_encryption_settings::de_hls_encryption_settings(tokens)?
                                );
                            }
                            "imageBasedTrickPlay" => {
                                builder = builder.set_image_based_trick_play(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsImageBasedTrickPlay::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "imageBasedTrickPlaySettings" => {
                                builder = builder.set_image_based_trick_play_settings(
                                    crate::protocol_serde::shape_hls_image_based_trick_play_settings::de_hls_image_based_trick_play_settings(tokens)?
                                );
                            }
                            "manifestCompression" => {
                                builder = builder.set_manifest_compression(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsManifestCompression::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "manifestDurationFormat" => {
                                builder = builder.set_manifest_duration_format(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsManifestDurationFormat::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "minFinalSegmentLength" => {
                                builder = builder.set_min_final_segment_length(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy())
                                );
                            }
                            "minSegmentLength" => {
                                builder = builder.set_min_segment_length(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "outputSelection" => {
                                builder = builder.set_output_selection(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsOutputSelection::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "programDateTime" => {
                                builder = builder.set_program_date_time(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsProgramDateTime::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "programDateTimePeriod" => {
                                builder = builder.set_program_date_time_period(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "segmentControl" => {
                                builder = builder.set_segment_control(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsSegmentControl::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "segmentLength" => {
                                builder = builder.set_segment_length(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "segmentLengthControl" => {
                                builder = builder.set_segment_length_control(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsSegmentLengthControl::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "segmentsPerSubdirectory" => {
                                builder = builder.set_segments_per_subdirectory(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "streamInfResolution" => {
                                builder = builder.set_stream_inf_resolution(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsStreamInfResolution::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "targetDurationCompatibilityMode" => {
                                builder = builder.set_target_duration_compatibility_mode(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsTargetDurationCompatibilityMode::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "timedMetadataId3Frame" => {
                                builder = builder.set_timed_metadata_id3_frame(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsTimedMetadataId3Frame::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "timedMetadataId3Period" => {
                                builder = builder.set_timed_metadata_id3_period(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "timestampDeltaMilliseconds" => {
                                builder = builder.set_timestamp_delta_milliseconds(
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

