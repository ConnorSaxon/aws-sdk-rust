// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_resources_input(input: &crate::input::ListResourcesInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_list_resources_input::ser_list_resources_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_resources_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListResourcesOutput, crate::error::ListResourcesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ListResourcesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ListResourcesError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AlreadyExistsException" => crate::error::ListResourcesError::AlreadyExistsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::already_exists_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_already_exists_exception::de_already_exists_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "GeneralServiceException" => crate::error::ListResourcesError::GeneralServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::general_service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_general_service_exception::de_general_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "HandlerFailureException" => crate::error::ListResourcesError::HandlerFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::handler_failure_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_handler_failure_exception::de_handler_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "HandlerInternalFailureException" => crate::error::ListResourcesError::HandlerInternalFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::handler_internal_failure_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_handler_internal_failure_exception::de_handler_internal_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidCredentialsException" => crate::error::ListResourcesError::InvalidCredentialsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_credentials_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_credentials_exception::de_invalid_credentials_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidRequestException" => crate::error::ListResourcesError::InvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NetworkFailureException" => crate::error::ListResourcesError::NetworkFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::network_failure_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_network_failure_exception::de_network_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotStabilizedException" => crate::error::ListResourcesError::NotStabilizedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::not_stabilized_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_stabilized_exception::de_not_stabilized_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotUpdatableException" => crate::error::ListResourcesError::NotUpdatableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::not_updatable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_updatable_exception::de_not_updatable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "PrivateTypeException" => crate::error::ListResourcesError::PrivateTypeException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::private_type_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_private_type_exception::de_private_type_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceConflictException" => crate::error::ListResourcesError::ResourceConflictException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_conflict_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_conflict_exception::de_resource_conflict_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::ListResourcesError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceInternalErrorException" => crate::error::ListResourcesError::ServiceInternalErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_internal_error_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_internal_error_exception::de_service_internal_error_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceLimitExceededException" => crate::error::ListResourcesError::ServiceLimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_limit_exceeded_exception::de_service_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ThrottlingException" => crate::error::ListResourcesError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TypeNotFoundException" => crate::error::ListResourcesError::TypeNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::type_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_type_not_found_exception::de_type_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedActionException" => crate::error::ListResourcesError::UnsupportedActionException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unsupported_action_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_action_exception::de_unsupported_action_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ListResourcesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_resources_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListResourcesOutput, crate::error::ListResourcesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_resources_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_list_resources::de_list_resources(response.body().as_ref(), output).map_err(crate::error::ListResourcesError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_list_resources(value: &[u8], mut builder: crate::output::list_resources_output::Builder) -> Result<crate::output::list_resources_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "TypeName" => {
                        builder = builder.set_type_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "ResourceDescriptions" => {
                        builder = builder.set_resource_descriptions(
                            crate::protocol_serde::shape_resource_descriptions::de_resource_descriptions(tokens)?
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

