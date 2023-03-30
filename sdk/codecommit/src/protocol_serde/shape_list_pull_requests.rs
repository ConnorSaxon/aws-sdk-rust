// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_pull_requests_input(input: &crate::input::ListPullRequestsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_list_pull_requests_input::ser_list_pull_requests_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_pull_requests_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListPullRequestsOutput, crate::error::ListPullRequestsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ListPullRequestsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ListPullRequestsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AuthorDoesNotExistException" => crate::error::ListPullRequestsError::AuthorDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::author_does_not_exist_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_author_does_not_exist_exception::de_author_does_not_exist_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListPullRequestsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionIntegrityChecksFailedException" => crate::error::ListPullRequestsError::EncryptionIntegrityChecksFailedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_integrity_checks_failed_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_integrity_checks_failed_exception::de_encryption_integrity_checks_failed_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListPullRequestsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionKeyAccessDeniedException" => crate::error::ListPullRequestsError::EncryptionKeyAccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_key_access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_key_access_denied_exception::de_encryption_key_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListPullRequestsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionKeyDisabledException" => crate::error::ListPullRequestsError::EncryptionKeyDisabledException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_key_disabled_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_key_disabled_exception::de_encryption_key_disabled_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListPullRequestsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionKeyNotFoundException" => crate::error::ListPullRequestsError::EncryptionKeyNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_key_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_key_not_found_exception::de_encryption_key_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListPullRequestsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionKeyUnavailableException" => crate::error::ListPullRequestsError::EncryptionKeyUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_key_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_key_unavailable_exception::de_encryption_key_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListPullRequestsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidAuthorArnException" => crate::error::ListPullRequestsError::InvalidAuthorArnException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_author_arn_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_author_arn_exception::de_invalid_author_arn_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListPullRequestsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidContinuationTokenException" => crate::error::ListPullRequestsError::InvalidContinuationTokenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_continuation_token_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_continuation_token_exception::de_invalid_continuation_token_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListPullRequestsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidMaxResultsException" => crate::error::ListPullRequestsError::InvalidMaxResultsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_max_results_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_max_results_exception::de_invalid_max_results_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListPullRequestsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidPullRequestStatusException" => crate::error::ListPullRequestsError::InvalidPullRequestStatusException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_pull_request_status_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_pull_request_status_exception::de_invalid_pull_request_status_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListPullRequestsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidRepositoryNameException" => crate::error::ListPullRequestsError::InvalidRepositoryNameException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_repository_name_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_repository_name_exception::de_invalid_repository_name_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListPullRequestsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "RepositoryDoesNotExistException" => crate::error::ListPullRequestsError::RepositoryDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::repository_does_not_exist_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_repository_does_not_exist_exception::de_repository_does_not_exist_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListPullRequestsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "RepositoryNameRequiredException" => crate::error::ListPullRequestsError::RepositoryNameRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::repository_name_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_repository_name_required_exception::de_repository_name_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListPullRequestsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ListPullRequestsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_pull_requests_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListPullRequestsOutput, crate::error::ListPullRequestsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_pull_requests_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_list_pull_requests::de_list_pull_requests(response.body().as_ref(), output).map_err(crate::error::ListPullRequestsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_list_pull_requests(value: &[u8], mut builder: crate::output::list_pull_requests_output::Builder) -> Result<crate::output::list_pull_requests_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "pullRequestIds" => {
                        builder = builder.set_pull_request_ids(
                            crate::protocol_serde::shape_pull_request_id_list::de_pull_request_id_list(tokens)?
                        );
                    }
                    "nextToken" => {
                        builder = builder.set_next_token(
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

