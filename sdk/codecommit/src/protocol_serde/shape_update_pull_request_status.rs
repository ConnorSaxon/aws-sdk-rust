// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_pull_request_status_input(input: &crate::input::UpdatePullRequestStatusInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_pull_request_status_input::ser_update_pull_request_status_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_pull_request_status_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdatePullRequestStatusOutput, crate::error::UpdatePullRequestStatusError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::UpdatePullRequestStatusError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UpdatePullRequestStatusError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "EncryptionIntegrityChecksFailedException" => crate::error::UpdatePullRequestStatusError::EncryptionIntegrityChecksFailedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_integrity_checks_failed_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_integrity_checks_failed_exception::de_encryption_integrity_checks_failed_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdatePullRequestStatusError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionKeyAccessDeniedException" => crate::error::UpdatePullRequestStatusError::EncryptionKeyAccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_key_access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_key_access_denied_exception::de_encryption_key_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdatePullRequestStatusError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionKeyDisabledException" => crate::error::UpdatePullRequestStatusError::EncryptionKeyDisabledException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_key_disabled_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_key_disabled_exception::de_encryption_key_disabled_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdatePullRequestStatusError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionKeyNotFoundException" => crate::error::UpdatePullRequestStatusError::EncryptionKeyNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_key_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_key_not_found_exception::de_encryption_key_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdatePullRequestStatusError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionKeyUnavailableException" => crate::error::UpdatePullRequestStatusError::EncryptionKeyUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_key_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_key_unavailable_exception::de_encryption_key_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdatePullRequestStatusError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidPullRequestIdException" => crate::error::UpdatePullRequestStatusError::InvalidPullRequestIdException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_pull_request_id_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_pull_request_id_exception::de_invalid_pull_request_id_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdatePullRequestStatusError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidPullRequestStatusException" => crate::error::UpdatePullRequestStatusError::InvalidPullRequestStatusException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_pull_request_status_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_pull_request_status_exception::de_invalid_pull_request_status_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdatePullRequestStatusError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidPullRequestStatusUpdateException" => crate::error::UpdatePullRequestStatusError::InvalidPullRequestStatusUpdateException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_pull_request_status_update_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_pull_request_status_update_exception::de_invalid_pull_request_status_update_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdatePullRequestStatusError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "PullRequestDoesNotExistException" => crate::error::UpdatePullRequestStatusError::PullRequestDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::pull_request_does_not_exist_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_pull_request_does_not_exist_exception::de_pull_request_does_not_exist_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdatePullRequestStatusError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "PullRequestIdRequiredException" => crate::error::UpdatePullRequestStatusError::PullRequestIdRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::pull_request_id_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_pull_request_id_required_exception::de_pull_request_id_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdatePullRequestStatusError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "PullRequestStatusRequiredException" => crate::error::UpdatePullRequestStatusError::PullRequestStatusRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::pull_request_status_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_pull_request_status_required_exception::de_pull_request_status_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdatePullRequestStatusError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::UpdatePullRequestStatusError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_pull_request_status_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdatePullRequestStatusOutput, crate::error::UpdatePullRequestStatusError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::update_pull_request_status_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_update_pull_request_status::de_update_pull_request_status(response.body().as_ref(), output).map_err(crate::error::UpdatePullRequestStatusError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_update_pull_request_status(value: &[u8], mut builder: crate::output::update_pull_request_status_output::Builder) -> Result<crate::output::update_pull_request_status_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
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

