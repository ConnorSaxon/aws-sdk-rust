// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_merge_commit_input(input: &crate::input::GetMergeCommitInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_merge_commit_input::ser_get_merge_commit_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_merge_commit_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetMergeCommitOutput, crate::error::GetMergeCommitError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetMergeCommitError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetMergeCommitError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "CommitDoesNotExistException" => crate::error::GetMergeCommitError::CommitDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::commit_does_not_exist_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_commit_does_not_exist_exception::de_commit_does_not_exist_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetMergeCommitError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CommitRequiredException" => crate::error::GetMergeCommitError::CommitRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::commit_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_commit_required_exception::de_commit_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetMergeCommitError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionIntegrityChecksFailedException" => crate::error::GetMergeCommitError::EncryptionIntegrityChecksFailedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_integrity_checks_failed_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_integrity_checks_failed_exception::de_encryption_integrity_checks_failed_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetMergeCommitError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionKeyAccessDeniedException" => crate::error::GetMergeCommitError::EncryptionKeyAccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_key_access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_key_access_denied_exception::de_encryption_key_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetMergeCommitError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionKeyDisabledException" => crate::error::GetMergeCommitError::EncryptionKeyDisabledException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_key_disabled_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_key_disabled_exception::de_encryption_key_disabled_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetMergeCommitError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionKeyNotFoundException" => crate::error::GetMergeCommitError::EncryptionKeyNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_key_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_key_not_found_exception::de_encryption_key_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetMergeCommitError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionKeyUnavailableException" => crate::error::GetMergeCommitError::EncryptionKeyUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_key_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_key_unavailable_exception::de_encryption_key_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetMergeCommitError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidCommitException" => crate::error::GetMergeCommitError::InvalidCommitException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_commit_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_commit_exception::de_invalid_commit_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetMergeCommitError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidConflictDetailLevelException" => crate::error::GetMergeCommitError::InvalidConflictDetailLevelException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_conflict_detail_level_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_conflict_detail_level_exception::de_invalid_conflict_detail_level_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetMergeCommitError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidConflictResolutionStrategyException" => crate::error::GetMergeCommitError::InvalidConflictResolutionStrategyException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_conflict_resolution_strategy_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_conflict_resolution_strategy_exception::de_invalid_conflict_resolution_strategy_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetMergeCommitError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidRepositoryNameException" => crate::error::GetMergeCommitError::InvalidRepositoryNameException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_repository_name_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_repository_name_exception::de_invalid_repository_name_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetMergeCommitError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "RepositoryDoesNotExistException" => crate::error::GetMergeCommitError::RepositoryDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::repository_does_not_exist_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_repository_does_not_exist_exception::de_repository_does_not_exist_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetMergeCommitError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "RepositoryNameRequiredException" => crate::error::GetMergeCommitError::RepositoryNameRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::repository_name_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_repository_name_required_exception::de_repository_name_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetMergeCommitError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GetMergeCommitError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_merge_commit_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetMergeCommitOutput, crate::error::GetMergeCommitError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_merge_commit_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_merge_commit::de_get_merge_commit(response.body().as_ref(), output).map_err(crate::error::GetMergeCommitError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_merge_commit(value: &[u8], mut builder: crate::output::get_merge_commit_output::Builder) -> Result<crate::output::get_merge_commit_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "sourceCommitId" => {
                        builder = builder.set_source_commit_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "destinationCommitId" => {
                        builder = builder.set_destination_commit_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "baseCommitId" => {
                        builder = builder.set_base_commit_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "mergedCommitId" => {
                        builder = builder.set_merged_commit_id(
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

