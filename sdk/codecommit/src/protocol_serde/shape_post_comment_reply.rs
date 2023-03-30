// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_post_comment_reply_input(input: &crate::input::PostCommentReplyInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_post_comment_reply_input::ser_post_comment_reply_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_post_comment_reply_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PostCommentReplyOutput, crate::error::PostCommentReplyError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::PostCommentReplyError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::PostCommentReplyError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ClientRequestTokenRequiredException" => crate::error::PostCommentReplyError::ClientRequestTokenRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::client_request_token_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_client_request_token_required_exception::de_client_request_token_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentReplyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CommentContentRequiredException" => crate::error::PostCommentReplyError::CommentContentRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::comment_content_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_comment_content_required_exception::de_comment_content_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentReplyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CommentContentSizeLimitExceededException" => crate::error::PostCommentReplyError::CommentContentSizeLimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::comment_content_size_limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_comment_content_size_limit_exceeded_exception::de_comment_content_size_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentReplyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CommentDoesNotExistException" => crate::error::PostCommentReplyError::CommentDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::comment_does_not_exist_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_comment_does_not_exist_exception::de_comment_does_not_exist_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentReplyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CommentIdRequiredException" => crate::error::PostCommentReplyError::CommentIdRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::comment_id_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_comment_id_required_exception::de_comment_id_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentReplyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "IdempotencyParameterMismatchException" => crate::error::PostCommentReplyError::IdempotencyParameterMismatchException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::idempotency_parameter_mismatch_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_idempotency_parameter_mismatch_exception::de_idempotency_parameter_mismatch_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentReplyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidClientRequestTokenException" => crate::error::PostCommentReplyError::InvalidClientRequestTokenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_client_request_token_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_client_request_token_exception::de_invalid_client_request_token_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentReplyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidCommentIdException" => crate::error::PostCommentReplyError::InvalidCommentIdException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_comment_id_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_comment_id_exception::de_invalid_comment_id_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PostCommentReplyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::PostCommentReplyError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_post_comment_reply_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PostCommentReplyOutput, crate::error::PostCommentReplyError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::post_comment_reply_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_post_comment_reply::de_post_comment_reply(response.body().as_ref(), output).map_err(crate::error::PostCommentReplyError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_post_comment_reply(value: &[u8], mut builder: crate::output::post_comment_reply_output::Builder) -> Result<crate::output::post_comment_reply_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
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

