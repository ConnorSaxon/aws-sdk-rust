// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_medical_stream_transcription_headers(
                    input: &crate::input::StartMedicalStreamTranscriptionInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.language_code {
        let formatted_2 = inner_1.as_str();
                        if !formatted_2.is_empty() {
                            let header_value = formatted_2;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("language_code", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amzn-transcribe-language-code", header_value);
                        }
    }
    if let Some(inner_3) = &input.media_sample_rate_hertz {
        let mut encoder = aws_smithy_types::primitive::Encoder::from(*inner_3);
        let formatted_4 = encoder.encode();
                        if !formatted_4.is_empty() {
                            let header_value = formatted_4;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("media_sample_rate_hertz", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amzn-transcribe-sample-rate", header_value);
                        }
    }
    if let Some(inner_5) = &input.media_encoding {
        let formatted_6 = inner_5.as_str();
                        if !formatted_6.is_empty() {
                            let header_value = formatted_6;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("media_encoding", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amzn-transcribe-media-encoding", header_value);
                        }
    }
    if let Some(inner_7) = &input.vocabulary_name {
        let formatted_8 = inner_7.as_str();
                        if !formatted_8.is_empty() {
                            let header_value = formatted_8;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("vocabulary_name", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amzn-transcribe-vocabulary-name", header_value);
                        }
    }
    if let Some(inner_9) = &input.specialty {
        let formatted_10 = inner_9.as_str();
                        if !formatted_10.is_empty() {
                            let header_value = formatted_10;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("specialty", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amzn-transcribe-specialty", header_value);
                        }
    }
    if let Some(inner_11) = &input.r#type {
        let formatted_12 = inner_11.as_str();
                        if !formatted_12.is_empty() {
                            let header_value = formatted_12;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("r#type", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amzn-transcribe-type", header_value);
                        }
    }
    if input.show_speaker_label {
        let mut encoder = aws_smithy_types::primitive::Encoder::from(input.show_speaker_label);
        let formatted_13 = encoder.encode();
                        if !formatted_13.is_empty() {
                            let header_value = formatted_13;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("show_speaker_label", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amzn-transcribe-show-speaker-label", header_value);
                        }
    }
    if let Some(inner_14) = &input.session_id {
        let formatted_15 = inner_14.as_str();
                        if !formatted_15.is_empty() {
                            let header_value = formatted_15;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("session_id", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amzn-transcribe-session-id", header_value);
                        }
    }
    if input.enable_channel_identification {
        let mut encoder = aws_smithy_types::primitive::Encoder::from(input.enable_channel_identification);
        let formatted_16 = encoder.encode();
                        if !formatted_16.is_empty() {
                            let header_value = formatted_16;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("enable_channel_identification", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amzn-transcribe-enable-channel-identification", header_value);
                        }
    }
    if let Some(inner_17) = &input.number_of_channels {
        let mut encoder = aws_smithy_types::primitive::Encoder::from(*inner_17);
        let formatted_18 = encoder.encode();
                        if !formatted_18.is_empty() {
                            let header_value = formatted_18;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("number_of_channels", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amzn-transcribe-number-of-channels", header_value);
                        }
    }
    if let Some(inner_19) = &input.content_identification_type {
        let formatted_20 = inner_19.as_str();
                        if !formatted_20.is_empty() {
                            let header_value = formatted_20;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("content_identification_type", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amzn-transcribe-content-identification-type", header_value);
                        }
    }
    Ok(builder)
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_medical_stream_transcription_http_response(op_response: &mut aws_smithy_http::operation::Response) -> std::result::Result<crate::output::StartMedicalStreamTranscriptionOutput, crate::error::StartMedicalStreamTranscriptionError> {
    #[allow(unused_variables)]
    let (response, properties) = op_response.parts_mut();
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::start_medical_stream_transcription_output::Builder::default();
        let _ = response;
        output = output.set_content_identification_type(
            crate::protocol_serde::shape_start_medical_stream_transcription_output::de_content_identification_type_header(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse ContentIdentificationType from header `x-amzn-transcribe-content-identification-type"))?
        );
        output = output.set_enable_channel_identification(
            crate::protocol_serde::shape_start_medical_stream_transcription_output::de_enable_channel_identification_header(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse EnableChannelIdentification from header `x-amzn-transcribe-enable-channel-identification"))?
        );
        output = output.set_language_code(
            crate::protocol_serde::shape_start_medical_stream_transcription_output::de_language_code_header(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse LanguageCode from header `x-amzn-transcribe-language-code"))?
        );
        output = output.set_media_encoding(
            crate::protocol_serde::shape_start_medical_stream_transcription_output::de_media_encoding_header(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse MediaEncoding from header `x-amzn-transcribe-media-encoding"))?
        );
        output = output.set_media_sample_rate_hertz(
            crate::protocol_serde::shape_start_medical_stream_transcription_output::de_media_sample_rate_hertz_header(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse MediaSampleRateHertz from header `x-amzn-transcribe-sample-rate"))?
        );
        output = output.set_number_of_channels(
            crate::protocol_serde::shape_start_medical_stream_transcription_output::de_number_of_channels_header(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse NumberOfChannels from header `x-amzn-transcribe-number-of-channels"))?
        );
        output = output.set_request_id(
            crate::protocol_serde::shape_start_medical_stream_transcription_output::de_request_id_header(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse RequestId from header `x-amzn-request-id"))?
        );
        output = output.set_session_id(
            crate::protocol_serde::shape_start_medical_stream_transcription_output::de_session_id_header(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse SessionId from header `x-amzn-transcribe-session-id"))?
        );
        output = output.set_show_speaker_label(
            crate::protocol_serde::shape_start_medical_stream_transcription_output::de_show_speaker_label_header(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse ShowSpeakerLabel from header `x-amzn-transcribe-show-speaker-label"))?
        );
        output = output.set_specialty(
            crate::protocol_serde::shape_start_medical_stream_transcription_output::de_specialty_header(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse Specialty from header `x-amzn-transcribe-specialty"))?
        );
        output = output.set_transcript_result_stream(
            Some(crate::protocol_serde::shape_start_medical_stream_transcription_output::de_transcript_result_stream_payload(response.body_mut())?)
        );
        output = output.set_type(
            crate::protocol_serde::shape_start_medical_stream_transcription_output::de_type_header(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse Type from header `x-amzn-transcribe-type"))?
        );
        output = output.set_vocabulary_name(
            crate::protocol_serde::shape_start_medical_stream_transcription_output::de_vocabulary_name_header(response.headers())
                                    .map_err(|_|crate::error::StartMedicalStreamTranscriptionError::unhandled("Failed to parse VocabularyName from header `x-amzn-transcribe-vocabulary-name"))?
        );
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build().map_err(crate::error::StartMedicalStreamTranscriptionError::unhandled)?
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_medical_stream_transcription_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StartMedicalStreamTranscriptionOutput, crate::error::StartMedicalStreamTranscriptionError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::StartMedicalStreamTranscriptionError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::StartMedicalStreamTranscriptionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ServiceUnavailableException" => crate::error::StartMedicalStreamTranscriptionError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartMedicalStreamTranscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "BadRequestException" => crate::error::StartMedicalStreamTranscriptionError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartMedicalStreamTranscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalFailureException" => crate::error::StartMedicalStreamTranscriptionError::InternalFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_failure_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_failure_exception::de_internal_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartMedicalStreamTranscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ConflictException" => crate::error::StartMedicalStreamTranscriptionError::ConflictException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::conflict_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartMedicalStreamTranscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceededException" => crate::error::StartMedicalStreamTranscriptionError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartMedicalStreamTranscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::StartMedicalStreamTranscriptionError::generic(generic)
    })
}

