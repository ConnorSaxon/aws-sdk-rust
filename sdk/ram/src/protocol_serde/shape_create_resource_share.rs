// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_resource_share_input(input: &crate::input::CreateResourceShareInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_resource_share_input::ser_create_resource_share_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_resource_share_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateResourceShareOutput, crate::error::CreateResourceShareError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateResourceShareError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateResourceShareError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "IdempotentParameterMismatchException" => crate::error::CreateResourceShareError::IdempotentParameterMismatchException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::idempotent_parameter_mismatch_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_idempotent_parameter_mismatch_exception::de_idempotent_parameter_mismatch_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateResourceShareError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidClientTokenException" => crate::error::CreateResourceShareError::InvalidClientTokenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_client_token_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_client_token_exception::de_invalid_client_token_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateResourceShareError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterException" => crate::error::CreateResourceShareError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateResourceShareError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidStateTransitionException" => crate::error::CreateResourceShareError::InvalidStateTransitionException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_state_transition_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_state_transition_exception::de_invalid_state_transition_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateResourceShareError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "MalformedArnException" => crate::error::CreateResourceShareError::MalformedArnException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::malformed_arn_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_malformed_arn_exception::de_malformed_arn_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateResourceShareError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OperationNotPermittedException" => crate::error::CreateResourceShareError::OperationNotPermittedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::operation_not_permitted_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_not_permitted_exception::de_operation_not_permitted_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateResourceShareError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceShareLimitExceededException" => crate::error::CreateResourceShareError::ResourceShareLimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_share_limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_share_limit_exceeded_exception::de_resource_share_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateResourceShareError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServerInternalException" => crate::error::CreateResourceShareError::ServerInternalException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::server_internal_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_server_internal_exception::de_server_internal_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateResourceShareError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceUnavailableException" => crate::error::CreateResourceShareError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateResourceShareError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TagPolicyViolationException" => crate::error::CreateResourceShareError::TagPolicyViolationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::tag_policy_violation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_tag_policy_violation_exception::de_tag_policy_violation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateResourceShareError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnknownResourceException" => crate::error::CreateResourceShareError::UnknownResourceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unknown_resource_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unknown_resource_exception::de_unknown_resource_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateResourceShareError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreateResourceShareError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_resource_share_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateResourceShareOutput, crate::error::CreateResourceShareError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_resource_share_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_resource_share::de_create_resource_share(response.body().as_ref(), output).map_err(crate::error::CreateResourceShareError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_create_resource_share(value: &[u8], mut builder: crate::output::create_resource_share_output::Builder) -> Result<crate::output::create_resource_share_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "clientToken" => {
                        builder = builder.set_client_token(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "resourceShare" => {
                        builder = builder.set_resource_share(
                            crate::protocol_serde::shape_resource_share::de_resource_share(tokens)?
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

