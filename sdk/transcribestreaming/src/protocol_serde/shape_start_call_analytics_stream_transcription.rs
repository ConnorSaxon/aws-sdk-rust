// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_call_analytics_stream_transcription_headers(
                    input: &crate::input::StartCallAnalyticsStreamTranscriptionInput,
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
    if let Some(inner_9) = &input.session_id {
        let formatted_10 = inner_9.as_str();
                        if !formatted_10.is_empty() {
                            let header_value = formatted_10;
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
    if let Some(inner_11) = &input.vocabulary_filter_name {
        let formatted_12 = inner_11.as_str();
                        if !formatted_12.is_empty() {
                            let header_value = formatted_12;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("vocabulary_filter_name", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amzn-transcribe-vocabulary-filter-name", header_value);
                        }
    }
    if let Some(inner_13) = &input.vocabulary_filter_method {
        let formatted_14 = inner_13.as_str();
                        if !formatted_14.is_empty() {
                            let header_value = formatted_14;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("vocabulary_filter_method", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amzn-transcribe-vocabulary-filter-method", header_value);
                        }
    }
    if let Some(inner_15) = &input.language_model_name {
        let formatted_16 = inner_15.as_str();
                        if !formatted_16.is_empty() {
                            let header_value = formatted_16;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("language_model_name", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amzn-transcribe-language-model-name", header_value);
                        }
    }
    if input.enable_partial_results_stabilization {
        let mut encoder = aws_smithy_types::primitive::Encoder::from(input.enable_partial_results_stabilization);
        let formatted_17 = encoder.encode();
                        if !formatted_17.is_empty() {
                            let header_value = formatted_17;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("enable_partial_results_stabilization", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amzn-transcribe-enable-partial-results-stabilization", header_value);
                        }
    }
    if let Some(inner_18) = &input.partial_results_stability {
        let formatted_19 = inner_18.as_str();
                        if !formatted_19.is_empty() {
                            let header_value = formatted_19;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("partial_results_stability", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amzn-transcribe-partial-results-stability", header_value);
                        }
    }
    if let Some(inner_20) = &input.content_identification_type {
        let formatted_21 = inner_20.as_str();
                        if !formatted_21.is_empty() {
                            let header_value = formatted_21;
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
    if let Some(inner_22) = &input.content_redaction_type {
        let formatted_23 = inner_22.as_str();
                        if !formatted_23.is_empty() {
                            let header_value = formatted_23;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("content_redaction_type", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amzn-transcribe-content-redaction-type", header_value);
                        }
    }
    if let Some(inner_24) = &input.pii_entity_types {
        let formatted_25 = inner_24.as_str();
                        if !formatted_25.is_empty() {
                            let header_value = formatted_25;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("pii_entity_types", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amzn-transcribe-pii-entity-types", header_value);
                        }
    }
    Ok(builder)
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_call_analytics_stream_transcription_http_response(op_response: &mut aws_smithy_http::operation::Response) -> std::result::Result<crate::output::StartCallAnalyticsStreamTranscriptionOutput, crate::error::StartCallAnalyticsStreamTranscriptionError> {
    #[allow(unused_variables)]
    let (response, properties) = op_response.parts_mut();
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::start_call_analytics_stream_transcription_output::Builder::default();
        let _ = response;
        output = output.set_call_analytics_transcript_result_stream(
            Some(crate::protocol_serde::shape_start_call_analytics_stream_transcription_output::de_call_analytics_transcript_result_stream_payload(response.body_mut())?)
        );
        output = output.set_content_identification_type(
            crate::protocol_serde::shape_start_call_analytics_stream_transcription_output::de_content_identification_type_header(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse ContentIdentificationType from header `x-amzn-transcribe-content-identification-type"))?
        );
        output = output.set_content_redaction_type(
            crate::protocol_serde::shape_start_call_analytics_stream_transcription_output::de_content_redaction_type_header(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse ContentRedactionType from header `x-amzn-transcribe-content-redaction-type"))?
        );
        output = output.set_enable_partial_results_stabilization(
            crate::protocol_serde::shape_start_call_analytics_stream_transcription_output::de_enable_partial_results_stabilization_header(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse EnablePartialResultsStabilization from header `x-amzn-transcribe-enable-partial-results-stabilization"))?
        );
        output = output.set_language_code(
            crate::protocol_serde::shape_start_call_analytics_stream_transcription_output::de_language_code_header(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse LanguageCode from header `x-amzn-transcribe-language-code"))?
        );
        output = output.set_language_model_name(
            crate::protocol_serde::shape_start_call_analytics_stream_transcription_output::de_language_model_name_header(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse LanguageModelName from header `x-amzn-transcribe-language-model-name"))?
        );
        output = output.set_media_encoding(
            crate::protocol_serde::shape_start_call_analytics_stream_transcription_output::de_media_encoding_header(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse MediaEncoding from header `x-amzn-transcribe-media-encoding"))?
        );
        output = output.set_media_sample_rate_hertz(
            crate::protocol_serde::shape_start_call_analytics_stream_transcription_output::de_media_sample_rate_hertz_header(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse MediaSampleRateHertz from header `x-amzn-transcribe-sample-rate"))?
        );
        output = output.set_partial_results_stability(
            crate::protocol_serde::shape_start_call_analytics_stream_transcription_output::de_partial_results_stability_header(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse PartialResultsStability from header `x-amzn-transcribe-partial-results-stability"))?
        );
        output = output.set_pii_entity_types(
            crate::protocol_serde::shape_start_call_analytics_stream_transcription_output::de_pii_entity_types_header(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse PiiEntityTypes from header `x-amzn-transcribe-pii-entity-types"))?
        );
        output = output.set_request_id(
            crate::protocol_serde::shape_start_call_analytics_stream_transcription_output::de_request_id_header(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse RequestId from header `x-amzn-request-id"))?
        );
        output = output.set_session_id(
            crate::protocol_serde::shape_start_call_analytics_stream_transcription_output::de_session_id_header(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse SessionId from header `x-amzn-transcribe-session-id"))?
        );
        output = output.set_vocabulary_filter_method(
            crate::protocol_serde::shape_start_call_analytics_stream_transcription_output::de_vocabulary_filter_method_header(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse VocabularyFilterMethod from header `x-amzn-transcribe-vocabulary-filter-method"))?
        );
        output = output.set_vocabulary_filter_name(
            crate::protocol_serde::shape_start_call_analytics_stream_transcription_output::de_vocabulary_filter_name_header(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse VocabularyFilterName from header `x-amzn-transcribe-vocabulary-filter-name"))?
        );
        output = output.set_vocabulary_name(
            crate::protocol_serde::shape_start_call_analytics_stream_transcription_output::de_vocabulary_name_header(response.headers())
                                    .map_err(|_|crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled("Failed to parse VocabularyName from header `x-amzn-transcribe-vocabulary-name"))?
        );
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build().map_err(crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled)?
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_call_analytics_stream_transcription_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StartCallAnalyticsStreamTranscriptionOutput, crate::error::StartCallAnalyticsStreamTranscriptionError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ServiceUnavailableException" => crate::error::StartCallAnalyticsStreamTranscriptionError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "BadRequestException" => crate::error::StartCallAnalyticsStreamTranscriptionError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalFailureException" => crate::error::StartCallAnalyticsStreamTranscriptionError::InternalFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_failure_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_failure_exception::de_internal_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ConflictException" => crate::error::StartCallAnalyticsStreamTranscriptionError::ConflictException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::conflict_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_conflict_exception::de_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceededException" => crate::error::StartCallAnalyticsStreamTranscriptionError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartCallAnalyticsStreamTranscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::StartCallAnalyticsStreamTranscriptionError::generic(generic)
    })
}

