// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_unpeer_vpc_input(_input: &crate::input::UnpeerVpcInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    Ok(aws_smithy_http::body::SdkBody::from("{}"))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_unpeer_vpc_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UnpeerVpcOutput, crate::error::UnpeerVpcError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::UnpeerVpcError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UnpeerVpcError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::UnpeerVpcError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UnpeerVpcError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "AccountSetupInProgressException" => crate::error::UnpeerVpcError::AccountSetupInProgressException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::account_setup_in_progress_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_account_setup_in_progress_exception::de_account_setup_in_progress_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UnpeerVpcError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInputException" => crate::error::UnpeerVpcError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_input_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UnpeerVpcError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotFoundException" => crate::error::UnpeerVpcError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UnpeerVpcError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OperationFailureException" => crate::error::UnpeerVpcError::OperationFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::operation_failure_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_failure_exception::de_operation_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UnpeerVpcError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceException" => crate::error::UnpeerVpcError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UnpeerVpcError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnauthenticatedException" => crate::error::UnpeerVpcError::UnauthenticatedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unauthenticated_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unauthenticated_exception::de_unauthenticated_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UnpeerVpcError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::UnpeerVpcError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_unpeer_vpc_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UnpeerVpcOutput, crate::error::UnpeerVpcError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::unpeer_vpc_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_unpeer_vpc::de_unpeer_vpc(response.body().as_ref(), output).map_err(crate::error::UnpeerVpcError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_unpeer_vpc(value: &[u8], mut builder: crate::output::unpeer_vpc_output::Builder) -> Result<crate::output::unpeer_vpc_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "operation" => {
                        builder = builder.set_operation(
                            crate::protocol_serde::shape_operation::de_operation(tokens)?
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

