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
    if let Some(var_4) = &input.base_url_content {
        object.key("baseUrlContent").string(var_4.as_str());
    }
    if let Some(var_5) = &input.base_url_content1 {
        object.key("baseUrlContent1").string(var_5.as_str());
    }
    if let Some(var_6) = &input.base_url_manifest {
        object.key("baseUrlManifest").string(var_6.as_str());
    }
    if let Some(var_7) = &input.base_url_manifest1 {
        object.key("baseUrlManifest1").string(var_7.as_str());
    }
    if let Some(var_8) = &input.caption_language_mappings {
        let mut array_9 = object.key("captionLanguageMappings").start_array();
        for item_10 in var_8 {
             {
                #[allow(unused_mut)]
                let mut object_11 = array_9.value().start_object();
                crate::protocol_serde::shape_caption_language_mapping::ser_caption_language_mapping(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    if let Some(var_12) = &input.caption_language_setting {
        object.key("captionLanguageSetting").string(var_12.as_str());
    }
    if let Some(var_13) = &input.client_cache {
        object.key("clientCache").string(var_13.as_str());
    }
    if let Some(var_14) = &input.codec_specification {
        object.key("codecSpecification").string(var_14.as_str());
    }
    if let Some(var_15) = &input.constant_iv {
        object.key("constantIv").string(var_15.as_str());
    }
    if let Some(var_16) = &input.destination {
        #[allow(unused_mut)]
        let mut object_17 = object.key("destination").start_object();
        crate::protocol_serde::shape_output_location_ref::ser_output_location_ref(&mut object_17, var_16)?;
        object_17.finish();
    }
    if let Some(var_18) = &input.directory_structure {
        object.key("directoryStructure").string(var_18.as_str());
    }
    if let Some(var_19) = &input.discontinuity_tags {
        object.key("discontinuityTags").string(var_19.as_str());
    }
    if let Some(var_20) = &input.encryption_type {
        object.key("encryptionType").string(var_20.as_str());
    }
    if let Some(var_21) = &input.hls_cdn_settings {
        #[allow(unused_mut)]
        let mut object_22 = object.key("hlsCdnSettings").start_object();
        crate::protocol_serde::shape_hls_cdn_settings::ser_hls_cdn_settings(&mut object_22, var_21)?;
        object_22.finish();
    }
    if let Some(var_23) = &input.hls_id3_segment_tagging {
        object.key("hlsId3SegmentTagging").string(var_23.as_str());
    }
    if let Some(var_24) = &input.i_frame_only_playlists {
        object.key("iFrameOnlyPlaylists").string(var_24.as_str());
    }
    if let Some(var_25) = &input.incomplete_segment_behavior {
        object.key("incompleteSegmentBehavior").string(var_25.as_str());
    }
    if input.index_n_segments != 0 {
        object.key("indexNSegments").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.index_n_segments).into()));
    }
    if let Some(var_26) = &input.input_loss_action {
        object.key("inputLossAction").string(var_26.as_str());
    }
    if let Some(var_27) = &input.iv_in_manifest {
        object.key("ivInManifest").string(var_27.as_str());
    }
    if let Some(var_28) = &input.iv_source {
        object.key("ivSource").string(var_28.as_str());
    }
    if input.keep_segments != 0 {
        object.key("keepSegments").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.keep_segments).into()));
    }
    if let Some(var_29) = &input.key_format {
        object.key("keyFormat").string(var_29.as_str());
    }
    if let Some(var_30) = &input.key_format_versions {
        object.key("keyFormatVersions").string(var_30.as_str());
    }
    if let Some(var_31) = &input.key_provider_settings {
        #[allow(unused_mut)]
        let mut object_32 = object.key("keyProviderSettings").start_object();
        crate::protocol_serde::shape_key_provider_settings::ser_key_provider_settings(&mut object_32, var_31)?;
        object_32.finish();
    }
    if let Some(var_33) = &input.manifest_compression {
        object.key("manifestCompression").string(var_33.as_str());
    }
    if let Some(var_34) = &input.manifest_duration_format {
        object.key("manifestDurationFormat").string(var_34.as_str());
    }
    if input.min_segment_length != 0 {
        object.key("minSegmentLength").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.min_segment_length).into()));
    }
    if let Some(var_35) = &input.mode {
        object.key("mode").string(var_35.as_str());
    }
    if let Some(var_36) = &input.output_selection {
        object.key("outputSelection").string(var_36.as_str());
    }
    if let Some(var_37) = &input.program_date_time {
        object.key("programDateTime").string(var_37.as_str());
    }
    if let Some(var_38) = &input.program_date_time_clock {
        object.key("programDateTimeClock").string(var_38.as_str());
    }
    if input.program_date_time_period != 0 {
        object.key("programDateTimePeriod").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.program_date_time_period).into()));
    }
    if let Some(var_39) = &input.redundant_manifest {
        object.key("redundantManifest").string(var_39.as_str());
    }
    if input.segment_length != 0 {
        object.key("segmentLength").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.segment_length).into()));
    }
    if let Some(var_40) = &input.segmentation_mode {
        object.key("segmentationMode").string(var_40.as_str());
    }
    if input.segments_per_subdirectory != 0 {
        object.key("segmentsPerSubdirectory").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.segments_per_subdirectory).into()));
    }
    if let Some(var_41) = &input.stream_inf_resolution {
        object.key("streamInfResolution").string(var_41.as_str());
    }
    if let Some(var_42) = &input.timed_metadata_id3_frame {
        object.key("timedMetadataId3Frame").string(var_42.as_str());
    }
    if input.timed_metadata_id3_period != 0 {
        object.key("timedMetadataId3Period").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.timed_metadata_id3_period).into()));
    }
    if input.timestamp_delta_milliseconds != 0 {
        object.key("timestampDeltaMilliseconds").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.timestamp_delta_milliseconds).into()));
    }
    if let Some(var_43) = &input.ts_file_mode {
        object.key("tsFileMode").string(var_43.as_str());
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
                            "baseUrlContent" => {
                                builder = builder.set_base_url_content(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "baseUrlContent1" => {
                                builder = builder.set_base_url_content1(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "baseUrlManifest" => {
                                builder = builder.set_base_url_manifest(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "baseUrlManifest1" => {
                                builder = builder.set_base_url_manifest1(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "captionLanguageMappings" => {
                                builder = builder.set_caption_language_mappings(
                                    crate::protocol_serde::shape___list_of_caption_language_mapping::de___list_of_caption_language_mapping(tokens)?
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
                            "constantIv" => {
                                builder = builder.set_constant_iv(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "destination" => {
                                builder = builder.set_destination(
                                    crate::protocol_serde::shape_output_location_ref::de_output_location_ref(tokens)?
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
                            "discontinuityTags" => {
                                builder = builder.set_discontinuity_tags(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsDiscontinuityTags::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "encryptionType" => {
                                builder = builder.set_encryption_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsEncryptionType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "hlsCdnSettings" => {
                                builder = builder.set_hls_cdn_settings(
                                    crate::protocol_serde::shape_hls_cdn_settings::de_hls_cdn_settings(tokens)?
                                );
                            }
                            "hlsId3SegmentTagging" => {
                                builder = builder.set_hls_id3_segment_tagging(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsId3SegmentTaggingState::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "iFrameOnlyPlaylists" => {
                                builder = builder.set_i_frame_only_playlists(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::IFrameOnlyPlaylistType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "incompleteSegmentBehavior" => {
                                builder = builder.set_incomplete_segment_behavior(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsIncompleteSegmentBehavior::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "indexNSegments" => {
                                builder = builder.set_index_n_segments(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "inputLossAction" => {
                                builder = builder.set_input_loss_action(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::InputLossActionForHlsOut::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "ivInManifest" => {
                                builder = builder.set_iv_in_manifest(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsIvInManifest::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "ivSource" => {
                                builder = builder.set_iv_source(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsIvSource::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "keepSegments" => {
                                builder = builder.set_keep_segments(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "keyFormat" => {
                                builder = builder.set_key_format(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "keyFormatVersions" => {
                                builder = builder.set_key_format_versions(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "keyProviderSettings" => {
                                builder = builder.set_key_provider_settings(
                                    crate::protocol_serde::shape_key_provider_settings::de_key_provider_settings(tokens)?
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
                            "minSegmentLength" => {
                                builder = builder.set_min_segment_length(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "mode" => {
                                builder = builder.set_mode(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsMode::from(u.as_ref())
                                        )
                                    ).transpose()?
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
                            "programDateTimeClock" => {
                                builder = builder.set_program_date_time_clock(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsProgramDateTimeClock::from(u.as_ref())
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
                            "redundantManifest" => {
                                builder = builder.set_redundant_manifest(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsRedundantManifest::from(u.as_ref())
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
                            "segmentationMode" => {
                                builder = builder.set_segmentation_mode(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsSegmentationMode::from(u.as_ref())
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
                            "tsFileMode" => {
                                builder = builder.set_ts_file_mode(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::HlsTsFileMode::from(u.as_ref())
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

