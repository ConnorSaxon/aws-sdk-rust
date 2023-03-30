// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_provisioned_concurrency_config_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetProvisionedConcurrencyConfigOutput, crate::error::GetProvisionedConcurrencyConfigError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetProvisionedConcurrencyConfigError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetProvisionedConcurrencyConfigError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidParameterValueException" => crate::error::GetProvisionedConcurrencyConfigError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_value_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetProvisionedConcurrencyConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ProvisionedConcurrencyConfigNotFoundException" => crate::error::GetProvisionedConcurrencyConfigError::ProvisionedConcurrencyConfigNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::provisioned_concurrency_config_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_provisioned_concurrency_config_not_found_exception::de_provisioned_concurrency_config_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetProvisionedConcurrencyConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::GetProvisionedConcurrencyConfigError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetProvisionedConcurrencyConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceException" => crate::error::GetProvisionedConcurrencyConfigError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetProvisionedConcurrencyConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRequestsException" => crate::error::GetProvisionedConcurrencyConfigError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_requests_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetProvisionedConcurrencyConfigError::unhandled)?;
                    output = output.set_retry_after_seconds(
                        crate::protocol_serde::shape_too_many_requests_exception::de_retry_after_seconds_header(response.headers())
                                                .map_err(|_|crate::error::GetProvisionedConcurrencyConfigError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After"))?
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
        _ => crate::error::GetProvisionedConcurrencyConfigError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_provisioned_concurrency_config_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetProvisionedConcurrencyConfigOutput, crate::error::GetProvisionedConcurrencyConfigError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_provisioned_concurrency_config_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_provisioned_concurrency_config::de_get_provisioned_concurrency_config(response.body().as_ref(), output).map_err(crate::error::GetProvisionedConcurrencyConfigError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_provisioned_concurrency_config(value: &[u8], mut builder: crate::output::get_provisioned_concurrency_config_output::Builder) -> Result<crate::output::get_provisioned_concurrency_config_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "AllocatedProvisionedConcurrentExecutions" => {
                        builder = builder.set_allocated_provisioned_concurrent_executions(
                            aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                .map(i32::try_from)
                                                .transpose()?
                        );
                    }
                    "AvailableProvisionedConcurrentExecutions" => {
                        builder = builder.set_available_provisioned_concurrent_executions(
                            aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                .map(i32::try_from)
                                                .transpose()?
                        );
                    }
                    "LastModified" => {
                        builder = builder.set_last_modified(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "RequestedProvisionedConcurrentExecutions" => {
                        builder = builder.set_requested_provisioned_concurrent_executions(
                            aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                .map(i32::try_from)
                                                .transpose()?
                        );
                    }
                    "Status" => {
                        builder = builder.set_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::ProvisionedConcurrencyStatusEnum::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "StatusReason" => {
                        builder = builder.set_status_reason(
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

