// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_post_comment_for_pull_request_input(input: &crate::input::PostCommentForPullRequestInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_post_comment_for_pull_request_input::ser_post_comment_for_pull_request_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_post_comment_for_pull_request_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PostCommentForPullRequestOutput, crate::error::PostCommentForPullRequestError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::PostCommentForPullRequestError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BeforeCommitIdAndAfterCommitIdAreSameException" => crate::error::PostCommentForPullRequestError::BeforeCommitIdAndAfterCommitIdAreSameException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::before_commit_id_and_after_commit_id_are_same_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_before_commit_id_and_after_commit_id_are_same_exception::de_before_commit_id_and_after_commit_id_are_same_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ClientRequestTokenRequiredException" => crate::error::PostCommentForPullRequestError::ClientRequestTokenRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::client_request_token_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_client_request_token_required_exception::de_client_request_token_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CommentContentRequiredException" => crate::error::PostCommentForPullRequestError::CommentContentRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::comment_content_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_comment_content_required_exception::de_comment_content_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CommentContentSizeLimitExceededException" => crate::error::PostCommentForPullRequestError::CommentContentSizeLimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::comment_content_size_limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_comment_content_size_limit_exceeded_exception::de_comment_content_size_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CommitDoesNotExistException" => crate::error::PostCommentForPullRequestError::CommitDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::commit_does_not_exist_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_commit_does_not_exist_exception::de_commit_does_not_exist_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CommitIdRequiredException" => crate::error::PostCommentForPullRequestError::CommitIdRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::commit_id_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_commit_id_required_exception::de_commit_id_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionIntegrityChecksFailedException" => crate::error::PostCommentForPullRequestError::EncryptionIntegrityChecksFailedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_integrity_checks_failed_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_integrity_checks_failed_exception::de_encryption_integrity_checks_failed_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionKeyAccessDeniedException" => crate::error::PostCommentForPullRequestError::EncryptionKeyAccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_key_access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_key_access_denied_exception::de_encryption_key_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionKeyDisabledException" => crate::error::PostCommentForPullRequestError::EncryptionKeyDisabledException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_key_disabled_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_key_disabled_exception::de_encryption_key_disabled_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionKeyNotFoundException" => crate::error::PostCommentForPullRequestError::EncryptionKeyNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_key_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_key_not_found_exception::de_encryption_key_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EncryptionKeyUnavailableException" => crate::error::PostCommentForPullRequestError::EncryptionKeyUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::encryption_key_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_encryption_key_unavailable_exception::de_encryption_key_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "IdempotencyParameterMismatchException" => crate::error::PostCommentForPullRequestError::IdempotencyParameterMismatchException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::idempotency_parameter_mismatch_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_idempotency_parameter_mismatch_exception::de_idempotency_parameter_mismatch_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidClientRequestTokenException" => crate::error::PostCommentForPullRequestError::InvalidClientRequestTokenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_client_request_token_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_client_request_token_exception::de_invalid_client_request_token_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidCommitIdException" => crate::error::PostCommentForPullRequestError::InvalidCommitIdException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_commit_id_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_commit_id_exception::de_invalid_commit_id_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidFileLocationException" => crate::error::PostCommentForPullRequestError::InvalidFileLocationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_file_location_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_file_location_exception::de_invalid_file_location_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidFilePositionException" => crate::error::PostCommentForPullRequestError::InvalidFilePositionException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_file_position_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_file_position_exception::de_invalid_file_position_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidPathException" => crate::error::PostCommentForPullRequestError::InvalidPathException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_path_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_path_exception::de_invalid_path_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidPullRequestIdException" => crate::error::PostCommentForPullRequestError::InvalidPullRequestIdException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_pull_request_id_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_pull_request_id_exception::de_invalid_pull_request_id_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidRelativeFileVersionEnumException" => crate::error::PostCommentForPullRequestError::InvalidRelativeFileVersionEnumException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_relative_file_version_enum_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_relative_file_version_enum_exception::de_invalid_relative_file_version_enum_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidRepositoryNameException" => crate::error::PostCommentForPullRequestError::InvalidRepositoryNameException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_repository_name_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_repository_name_exception::de_invalid_repository_name_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "PathDoesNotExistException" => crate::error::PostCommentForPullRequestError::PathDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::path_does_not_exist_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_path_does_not_exist_exception::de_path_does_not_exist_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "PathRequiredException" => crate::error::PostCommentForPullRequestError::PathRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::path_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_path_required_exception::de_path_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "PullRequestDoesNotExistException" => crate::error::PostCommentForPullRequestError::PullRequestDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::pull_request_does_not_exist_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_pull_request_does_not_exist_exception::de_pull_request_does_not_exist_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "PullRequestIdRequiredException" => crate::error::PostCommentForPullRequestError::PullRequestIdRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::pull_request_id_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_pull_request_id_required_exception::de_pull_request_id_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "RepositoryDoesNotExistException" => crate::error::PostCommentForPullRequestError::RepositoryDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::repository_does_not_exist_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_repository_does_not_exist_exception::de_repository_does_not_exist_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "RepositoryNameRequiredException" => crate::error::PostCommentForPullRequestError::RepositoryNameRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::repository_name_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_repository_name_required_exception::de_repository_name_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "RepositoryNotAssociatedWithPullRequestException" => crate::error::PostCommentForPullRequestError::RepositoryNotAssociatedWithPullRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::repository_not_associated_with_pull_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_repository_not_associated_with_pull_request_exception::de_repository_not_associated_with_pull_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::PostCommentForPullRequestError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_post_comment_for_pull_request_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PostCommentForPullRequestOutput, crate::error::PostCommentForPullRequestError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::post_comment_for_pull_request_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_post_comment_for_pull_request::de_post_comment_for_pull_request(response.body().as_ref(), output).map_err(crate::error::PostCommentForPullRequestError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_post_comment_for_pull_request(value: &[u8], mut builder: crate::output::post_comment_for_pull_request_output::Builder) -> Result<crate::output::post_comment_for_pull_request_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "repositoryName" => {
                        builder = builder.set_repository_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "pullRequestId" => {
                        builder = builder.set_pull_request_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "beforeCommitId" => {
                        builder = builder.set_before_commit_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "afterCommitId" => {
                        builder = builder.set_after_commit_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "beforeBlobId" => {
                        builder = builder.set_before_blob_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "afterBlobId" => {
                        builder = builder.set_after_blob_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "location" => {
                        builder = builder.set_location(
                            crate::protocol_serde::shape_location::de_location(tokens)?
                        );
                    }
                    "comment" => {
                        builder = builder.set_comment(
                            crate::protocol_serde::shape_comment::de_comment(tokens)?
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

