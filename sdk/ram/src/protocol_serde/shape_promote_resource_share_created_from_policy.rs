// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_promote_resource_share_created_from_policy_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PromoteResourceShareCreatedFromPolicyOutput, crate::error::PromoteResourceShareCreatedFromPolicyError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::PromoteResourceShareCreatedFromPolicyError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::PromoteResourceShareCreatedFromPolicyError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidParameterException" => crate::error::PromoteResourceShareCreatedFromPolicyError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PromoteResourceShareCreatedFromPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "MalformedArnException" => crate::error::PromoteResourceShareCreatedFromPolicyError::MalformedArnException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::malformed_arn_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_malformed_arn_exception::de_malformed_arn_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PromoteResourceShareCreatedFromPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "MissingRequiredParameterException" => crate::error::PromoteResourceShareCreatedFromPolicyError::MissingRequiredParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::missing_required_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_missing_required_parameter_exception::de_missing_required_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PromoteResourceShareCreatedFromPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OperationNotPermittedException" => crate::error::PromoteResourceShareCreatedFromPolicyError::OperationNotPermittedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::operation_not_permitted_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_not_permitted_exception::de_operation_not_permitted_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PromoteResourceShareCreatedFromPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceShareLimitExceededException" => crate::error::PromoteResourceShareCreatedFromPolicyError::ResourceShareLimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_share_limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_share_limit_exceeded_exception::de_resource_share_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PromoteResourceShareCreatedFromPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServerInternalException" => crate::error::PromoteResourceShareCreatedFromPolicyError::ServerInternalException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::server_internal_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_server_internal_exception::de_server_internal_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PromoteResourceShareCreatedFromPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceUnavailableException" => crate::error::PromoteResourceShareCreatedFromPolicyError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PromoteResourceShareCreatedFromPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnknownResourceException" => crate::error::PromoteResourceShareCreatedFromPolicyError::UnknownResourceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unknown_resource_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unknown_resource_exception::de_unknown_resource_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PromoteResourceShareCreatedFromPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::PromoteResourceShareCreatedFromPolicyError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_promote_resource_share_created_from_policy_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PromoteResourceShareCreatedFromPolicyOutput, crate::error::PromoteResourceShareCreatedFromPolicyError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::promote_resource_share_created_from_policy_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_promote_resource_share_created_from_policy::de_promote_resource_share_created_from_policy(response.body().as_ref(), output).map_err(crate::error::PromoteResourceShareCreatedFromPolicyError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_promote_resource_share_created_from_policy(value: &[u8], mut builder: crate::output::promote_resource_share_created_from_policy_output::Builder) -> Result<crate::output::promote_resource_share_created_from_policy_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "returnValue" => {
                        builder = builder.set_return_value(
                            aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
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

