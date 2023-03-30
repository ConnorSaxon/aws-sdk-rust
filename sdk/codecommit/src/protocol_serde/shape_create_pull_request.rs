// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_pull_request_input(input: &crate::input::CreatePullRequestInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_pull_request_input::ser_create_pull_request_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_pull_request_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreatePullRequestOutput, crate::error::CreatePullRequestError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreatePullRequestError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreatePullRequestError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ClientRequestTokenRequiredException" => crate::error::CreatePullRequestError::ClientRequestTokenRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::client_request_token_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_client_request_token_required_exception::de_client_request_token_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionIntegrityChecksFailedException" => crate::error::CreatePullRequestError::EncryptionIntegrityChecksFailedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_integrity_checks_failed_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_integrity_checks_failed_exception::de_encryption_integrity_checks_failed_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionKeyAccessDeniedException" => crate::error::CreatePullRequestError::EncryptionKeyAccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_key_access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_key_access_denied_exception::de_encryption_key_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionKeyDisabledException" => crate::error::CreatePullRequestError::EncryptionKeyDisabledException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_key_disabled_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_key_disabled_exception::de_encryption_key_disabled_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionKeyNotFoundException" => crate::error::CreatePullRequestError::EncryptionKeyNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_key_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_key_not_found_exception::de_encryption_key_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionKeyUnavailableException" => crate::error::CreatePullRequestError::EncryptionKeyUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_key_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_key_unavailable_exception::de_encryption_key_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "IdempotencyParameterMismatchException" => crate::error::CreatePullRequestError::IdempotencyParameterMismatchException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::idempotency_parameter_mismatch_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_idempotency_parameter_mismatch_exception::de_idempotency_parameter_mismatch_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidClientRequestTokenException" => crate::error::CreatePullRequestError::InvalidClientRequestTokenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_client_request_token_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_client_request_token_exception::de_invalid_client_request_token_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidDescriptionException" => crate::error::CreatePullRequestError::InvalidDescriptionException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_description_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_description_exception::de_invalid_description_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidReferenceNameException" => crate::error::CreatePullRequestError::InvalidReferenceNameException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_reference_name_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_reference_name_exception::de_invalid_reference_name_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidRepositoryNameException" => crate::error::CreatePullRequestError::InvalidRepositoryNameException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_repository_name_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_repository_name_exception::de_invalid_repository_name_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidTargetException" => crate::error::CreatePullRequestError::InvalidTargetException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_target_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_target_exception::de_invalid_target_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidTargetsException" => crate::error::CreatePullRequestError::InvalidTargetsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_targets_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_targets_exception::de_invalid_targets_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidTitleException" => crate::error::CreatePullRequestError::InvalidTitleException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_title_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_title_exception::de_invalid_title_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "MaximumOpenPullRequestsExceededException" => crate::error::CreatePullRequestError::MaximumOpenPullRequestsExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::maximum_open_pull_requests_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_maximum_open_pull_requests_exceeded_exception::de_maximum_open_pull_requests_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "MultipleRepositoriesInPullRequestException" => crate::error::CreatePullRequestError::MultipleRepositoriesInPullRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::multiple_repositories_in_pull_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_multiple_repositories_in_pull_request_exception::de_multiple_repositories_in_pull_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ReferenceDoesNotExistException" => crate::error::CreatePullRequestError::ReferenceDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::reference_does_not_exist_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_reference_does_not_exist_exception::de_reference_does_not_exist_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ReferenceNameRequiredException" => crate::error::CreatePullRequestError::ReferenceNameRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::reference_name_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_reference_name_required_exception::de_reference_name_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ReferenceTypeNotSupportedException" => crate::error::CreatePullRequestError::ReferenceTypeNotSupportedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::reference_type_not_supported_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_reference_type_not_supported_exception::de_reference_type_not_supported_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "RepositoryDoesNotExistException" => crate::error::CreatePullRequestError::RepositoryDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::repository_does_not_exist_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_repository_does_not_exist_exception::de_repository_does_not_exist_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "RepositoryNameRequiredException" => crate::error::CreatePullRequestError::RepositoryNameRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::repository_name_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_repository_name_required_exception::de_repository_name_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "SourceAndDestinationAreSameException" => crate::error::CreatePullRequestError::SourceAndDestinationAreSameException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::source_and_destination_are_same_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_source_and_destination_are_same_exception::de_source_and_destination_are_same_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TargetRequiredException" => crate::error::CreatePullRequestError::TargetRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::target_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_target_required_exception::de_target_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TargetsRequiredException" => crate::error::CreatePullRequestError::TargetsRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::targets_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_targets_required_exception::de_targets_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TitleRequiredException" => crate::error::CreatePullRequestError::TitleRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::title_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_title_required_exception::de_title_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreatePullRequestError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_pull_request_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreatePullRequestOutput, crate::error::CreatePullRequestError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_pull_request_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_pull_request::de_create_pull_request(response.body().as_ref(), output).map_err(crate::error::CreatePullRequestError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_create_pull_request(value: &[u8], mut builder: crate::output::create_pull_request_output::Builder) -> Result<crate::output::create_pull_request_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "pullRequest" => {
                        builder = builder.set_pull_request(
                            crate::protocol_serde::shape_pull_request::de_pull_request(tokens)?
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

