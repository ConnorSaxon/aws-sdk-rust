// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_playback_configuration_input(input: &crate::input::PutPlaybackConfigurationInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_put_playback_configuration_input::ser_put_playback_configuration_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_playback_configuration_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PutPlaybackConfigurationOutput, crate::error::PutPlaybackConfigurationError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::PutPlaybackConfigurationError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    Err(crate::error::PutPlaybackConfigurationError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_playback_configuration_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PutPlaybackConfigurationOutput, crate::error::PutPlaybackConfigurationError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::put_playback_configuration_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_put_playback_configuration::de_put_playback_configuration(response.body().as_ref(), output).map_err(crate::error::PutPlaybackConfigurationError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_put_playback_configuration(value: &[u8], mut builder: crate::output::put_playback_configuration_output::Builder) -> Result<crate::output::put_playback_configuration_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "AdDecisionServerUrl" => {
                        builder = builder.set_ad_decision_server_url(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "AvailSuppression" => {
                        builder = builder.set_avail_suppression(
                            crate::protocol_serde::shape_avail_suppression::de_avail_suppression(tokens)?
                        );
                    }
                    "Bumper" => {
                        builder = builder.set_bumper(
                            crate::protocol_serde::shape_bumper::de_bumper(tokens)?
                        );
                    }
                    "CdnConfiguration" => {
                        builder = builder.set_cdn_configuration(
                            crate::protocol_serde::shape_cdn_configuration::de_cdn_configuration(tokens)?
                        );
                    }
                    "ConfigurationAliases" => {
                        builder = builder.set_configuration_aliases(
                            crate::protocol_serde::shape_configuration_aliases_response::de_configuration_aliases_response(tokens)?
                        );
                    }
                    "DashConfiguration" => {
                        builder = builder.set_dash_configuration(
                            crate::protocol_serde::shape_dash_configuration::de_dash_configuration(tokens)?
                        );
                    }
                    "HlsConfiguration" => {
                        builder = builder.set_hls_configuration(
                            crate::protocol_serde::shape_hls_configuration::de_hls_configuration(tokens)?
                        );
                    }
                    "LivePreRollConfiguration" => {
                        builder = builder.set_live_pre_roll_configuration(
                            crate::protocol_serde::shape_live_pre_roll_configuration::de_live_pre_roll_configuration(tokens)?
                        );
                    }
                    "LogConfiguration" => {
                        builder = builder.set_log_configuration(
                            crate::protocol_serde::shape_log_configuration::de_log_configuration(tokens)?
                        );
                    }
                    "ManifestProcessingRules" => {
                        builder = builder.set_manifest_processing_rules(
                            crate::protocol_serde::shape_manifest_processing_rules::de_manifest_processing_rules(tokens)?
                        );
                    }
                    "Name" => {
                        builder = builder.set_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "PersonalizationThresholdSeconds" => {
                        builder = builder.set_personalization_threshold_seconds(
                            aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                .map(i32::try_from)
                                                .transpose()?
                        );
                    }
                    "PlaybackConfigurationArn" => {
                        builder = builder.set_playback_configuration_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "PlaybackEndpointPrefix" => {
                        builder = builder.set_playback_endpoint_prefix(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "SessionInitializationEndpointPrefix" => {
                        builder = builder.set_session_initialization_endpoint_prefix(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "SlateAdUrl" => {
                        builder = builder.set_slate_ad_url(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "tags" => {
                        builder = builder.set_tags(
                            crate::protocol_serde::shape___map_of__string::de___map_of__string(tokens)?
                        );
                    }
                    "TranscodeProfileName" => {
                        builder = builder.set_transcode_profile_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "VideoContentSourceUrl" => {
                        builder = builder.set_video_content_source_url(
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
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("found more JSON tokens after completing parsing"));
    }
    Ok(builder)
}

