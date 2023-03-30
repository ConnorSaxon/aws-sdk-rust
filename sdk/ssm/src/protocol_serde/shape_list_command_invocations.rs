// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_command_invocations_input(input: &crate::input::ListCommandInvocationsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_list_command_invocations_input::ser_list_command_invocations_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_command_invocations_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListCommandInvocationsOutput, crate::error::ListCommandInvocationsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ListCommandInvocationsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ListCommandInvocationsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InternalServerError" => crate::error::ListCommandInvocationsError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(response.body().as_ref(), output).map_err(crate::error::ListCommandInvocationsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidCommandId" => crate::error::ListCommandInvocationsError::InvalidCommandId({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_command_id::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_command_id::de_invalid_command_id_json_err(response.body().as_ref(), output).map_err(crate::error::ListCommandInvocationsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidFilterKey" => crate::error::ListCommandInvocationsError::InvalidFilterKey({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_filter_key::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_filter_key::de_invalid_filter_key_json_err(response.body().as_ref(), output).map_err(crate::error::ListCommandInvocationsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInstanceId" => crate::error::ListCommandInvocationsError::InvalidInstanceId({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_instance_id::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_instance_id::de_invalid_instance_id_json_err(response.body().as_ref(), output).map_err(crate::error::ListCommandInvocationsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidNextToken" => crate::error::ListCommandInvocationsError::InvalidNextToken({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_next_token::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_next_token::de_invalid_next_token_json_err(response.body().as_ref(), output).map_err(crate::error::ListCommandInvocationsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ListCommandInvocationsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_command_invocations_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListCommandInvocationsOutput, crate::error::ListCommandInvocationsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_command_invocations_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_list_command_invocations::de_list_command_invocations(response.body().as_ref(), output).map_err(crate::error::ListCommandInvocationsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_list_command_invocations(value: &[u8], mut builder: crate::output::list_command_invocations_output::Builder) -> Result<crate::output::list_command_invocations_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "CommandInvocations" => {
                        builder = builder.set_command_invocations(
                            crate::protocol_serde::shape_command_invocation_list::de_command_invocation_list(tokens)?
                        );
                    }
                    "NextToken" => {
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

