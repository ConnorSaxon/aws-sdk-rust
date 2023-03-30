// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_bot_aliases_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetBotAliasesOutput, crate::error::GetBotAliasesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetBotAliasesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetBotAliasesError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::error::GetBotAliasesError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetBotAliasesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalFailureException" => crate::error::GetBotAliasesError::InternalFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_failure_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_failure_exception::de_internal_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetBotAliasesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceededException" => crate::error::GetBotAliasesError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetBotAliasesError::unhandled)?;
                    output = output.set_retry_after_seconds(
                        crate::protocol_serde::shape_limit_exceeded_exception::de_retry_after_seconds_header(response.headers())
                                                .map_err(|_|crate::error::GetBotAliasesError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After"))?
                    );
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GetBotAliasesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_bot_aliases_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetBotAliasesOutput, crate::error::GetBotAliasesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_bot_aliases_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_bot_aliases::de_get_bot_aliases(response.body().as_ref(), output).map_err(crate::error::GetBotAliasesError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_bot_aliases(value: &[u8], mut builder: crate::output::get_bot_aliases_output::Builder) -> Result<crate::output::get_bot_aliases_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "BotAliases" => {
                        builder = builder.set_bot_aliases(
                            crate::protocol_serde::shape_bot_alias_metadata_list::de_bot_alias_metadata_list(tokens)?
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

