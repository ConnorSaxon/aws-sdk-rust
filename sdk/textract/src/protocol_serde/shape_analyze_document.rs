// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_analyze_document_input(input: &crate::input::AnalyzeDocumentInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_analyze_document_input::ser_analyze_document_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_analyze_document_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AnalyzeDocumentOutput, crate::error::AnalyzeDocumentError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::AnalyzeDocumentError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::AnalyzeDocumentError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::AnalyzeDocumentError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AnalyzeDocumentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "BadDocumentException" => crate::error::AnalyzeDocumentError::BadDocumentException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_document_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_document_exception::de_bad_document_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AnalyzeDocumentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DocumentTooLargeException" => crate::error::AnalyzeDocumentError::DocumentTooLargeException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::document_too_large_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_document_too_large_exception::de_document_too_large_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AnalyzeDocumentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "HumanLoopQuotaExceededException" => crate::error::AnalyzeDocumentError::HumanLoopQuotaExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::human_loop_quota_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_human_loop_quota_exceeded_exception::de_human_loop_quota_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AnalyzeDocumentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerError" => crate::error::AnalyzeDocumentError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(response.body().as_ref(), output).map_err(crate::error::AnalyzeDocumentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterException" => crate::error::AnalyzeDocumentError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AnalyzeDocumentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidS3ObjectException" => crate::error::AnalyzeDocumentError::InvalidS3ObjectException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_s3_object_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_s3_object_exception::de_invalid_s3_object_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AnalyzeDocumentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ProvisionedThroughputExceededException" => crate::error::AnalyzeDocumentError::ProvisionedThroughputExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::provisioned_throughput_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_provisioned_throughput_exceeded_exception::de_provisioned_throughput_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AnalyzeDocumentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ThrottlingException" => crate::error::AnalyzeDocumentError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AnalyzeDocumentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedDocumentException" => crate::error::AnalyzeDocumentError::UnsupportedDocumentException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unsupported_document_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_document_exception::de_unsupported_document_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AnalyzeDocumentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::AnalyzeDocumentError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_analyze_document_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AnalyzeDocumentOutput, crate::error::AnalyzeDocumentError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::analyze_document_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_analyze_document::de_analyze_document(response.body().as_ref(), output).map_err(crate::error::AnalyzeDocumentError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_analyze_document(value: &[u8], mut builder: crate::output::analyze_document_output::Builder) -> Result<crate::output::analyze_document_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "DocumentMetadata" => {
                        builder = builder.set_document_metadata(
                            crate::protocol_serde::shape_document_metadata::de_document_metadata(tokens)?
                        );
                    }
                    "Blocks" => {
                        builder = builder.set_blocks(
                            crate::protocol_serde::shape_block_list::de_block_list(tokens)?
                        );
                    }
                    "HumanLoopActivationOutput" => {
                        builder = builder.set_human_loop_activation_output(
                            crate::protocol_serde::shape_human_loop_activation_output::de_human_loop_activation_output(tokens)?
                        );
                    }
                    "AnalyzeDocumentModelVersion" => {
                        builder = builder.set_analyze_document_model_version(
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

