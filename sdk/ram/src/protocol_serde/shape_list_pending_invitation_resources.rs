// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_pending_invitation_resources_input(input: &crate::input::ListPendingInvitationResourcesInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_list_pending_invitation_resources_input::ser_list_pending_invitation_resources_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_pending_invitation_resources_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListPendingInvitationResourcesOutput, crate::error::ListPendingInvitationResourcesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ListPendingInvitationResourcesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ListPendingInvitationResourcesError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidNextTokenException" => crate::error::ListPendingInvitationResourcesError::InvalidNextTokenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_next_token_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListPendingInvitationResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterException" => crate::error::ListPendingInvitationResourcesError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListPendingInvitationResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "MalformedArnException" => crate::error::ListPendingInvitationResourcesError::MalformedArnException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::malformed_arn_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_malformed_arn_exception::de_malformed_arn_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListPendingInvitationResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "MissingRequiredParameterException" => crate::error::ListPendingInvitationResourcesError::MissingRequiredParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::missing_required_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_missing_required_parameter_exception::de_missing_required_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListPendingInvitationResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceShareInvitationAlreadyRejectedException" => crate::error::ListPendingInvitationResourcesError::ResourceShareInvitationAlreadyRejectedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_share_invitation_already_rejected_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_share_invitation_already_rejected_exception::de_resource_share_invitation_already_rejected_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListPendingInvitationResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceShareInvitationArnNotFoundException" => crate::error::ListPendingInvitationResourcesError::ResourceShareInvitationArnNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_share_invitation_arn_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_share_invitation_arn_not_found_exception::de_resource_share_invitation_arn_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListPendingInvitationResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceShareInvitationExpiredException" => crate::error::ListPendingInvitationResourcesError::ResourceShareInvitationExpiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_share_invitation_expired_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_share_invitation_expired_exception::de_resource_share_invitation_expired_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListPendingInvitationResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServerInternalException" => crate::error::ListPendingInvitationResourcesError::ServerInternalException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::server_internal_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_server_internal_exception::de_server_internal_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListPendingInvitationResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceUnavailableException" => crate::error::ListPendingInvitationResourcesError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListPendingInvitationResourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ListPendingInvitationResourcesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_pending_invitation_resources_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListPendingInvitationResourcesOutput, crate::error::ListPendingInvitationResourcesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_pending_invitation_resources_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_list_pending_invitation_resources::de_list_pending_invitation_resources(response.body().as_ref(), output).map_err(crate::error::ListPendingInvitationResourcesError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_list_pending_invitation_resources(value: &[u8], mut builder: crate::output::list_pending_invitation_resources_output::Builder) -> Result<crate::output::list_pending_invitation_resources_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "nextToken" => {
                        builder = builder.set_next_token(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "resources" => {
                        builder = builder.set_resources(
                            crate::protocol_serde::shape_resource_list::de_resource_list(tokens)?
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

