// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_attach_to_index_headers(
                    input: &crate::input::AttachToIndexInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.directory_arn {
        let formatted_2 = inner_1.as_str();
                        if !formatted_2.is_empty() {
                            let header_value = formatted_2;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("directory_arn", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("x-amz-data-partition", header_value);
                        }
    }
    Ok(builder)
}

pub fn ser_attach_to_index_input(input: &crate::input::AttachToIndexInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_attach_to_index_input::ser_attach_to_index_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_attach_to_index_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AttachToIndexOutput, crate::error::AttachToIndexError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::AttachToIndexError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::AttachToIndexError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::AttachToIndexError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AttachToIndexError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DirectoryNotEnabledException" => crate::error::AttachToIndexError::DirectoryNotEnabledException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::directory_not_enabled_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_directory_not_enabled_exception::de_directory_not_enabled_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AttachToIndexError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "IndexedAttributeMissingException" => crate::error::AttachToIndexError::IndexedAttributeMissingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::indexed_attribute_missing_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_indexed_attribute_missing_exception::de_indexed_attribute_missing_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AttachToIndexError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServiceException" => crate::error::AttachToIndexError::InternalServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_service_exception::de_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AttachToIndexError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidArnException" => crate::error::AttachToIndexError::InvalidArnException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_arn_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_arn_exception::de_invalid_arn_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AttachToIndexError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidAttachmentException" => crate::error::AttachToIndexError::InvalidAttachmentException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_attachment_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_attachment_exception::de_invalid_attachment_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AttachToIndexError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceededException" => crate::error::AttachToIndexError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AttachToIndexError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LinkNameAlreadyInUseException" => crate::error::AttachToIndexError::LinkNameAlreadyInUseException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::link_name_already_in_use_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_link_name_already_in_use_exception::de_link_name_already_in_use_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AttachToIndexError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotIndexException" => crate::error::AttachToIndexError::NotIndexException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::not_index_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_index_exception::de_not_index_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AttachToIndexError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::AttachToIndexError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AttachToIndexError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "RetryableConflictException" => crate::error::AttachToIndexError::RetryableConflictException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::retryable_conflict_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_retryable_conflict_exception::de_retryable_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AttachToIndexError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ValidationException" => crate::error::AttachToIndexError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AttachToIndexError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::AttachToIndexError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_attach_to_index_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AttachToIndexOutput, crate::error::AttachToIndexError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::attach_to_index_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_attach_to_index::de_attach_to_index(response.body().as_ref(), output).map_err(crate::error::AttachToIndexError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_attach_to_index(value: &[u8], mut builder: crate::output::attach_to_index_output::Builder) -> Result<crate::output::attach_to_index_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "AttachedObjectIdentifier" => {
                        builder = builder.set_attached_object_identifier(
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

