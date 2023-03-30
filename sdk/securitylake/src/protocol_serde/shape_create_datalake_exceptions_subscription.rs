// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_datalake_exceptions_subscription_input(input: &crate::input::CreateDatalakeExceptionsSubscriptionInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_datalake_exceptions_subscription_input::ser_create_datalake_exceptions_subscription_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_datalake_exceptions_subscription_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateDatalakeExceptionsSubscriptionOutput, crate::error::CreateDatalakeExceptionsSubscriptionError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateDatalakeExceptionsSubscriptionError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateDatalakeExceptionsSubscriptionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::CreateDatalakeExceptionsSubscriptionError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateDatalakeExceptionsSubscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "AccountNotFoundException" => crate::error::CreateDatalakeExceptionsSubscriptionError::AccountNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::account_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_account_not_found_exception::de_account_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateDatalakeExceptionsSubscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerException" => crate::error::CreateDatalakeExceptionsSubscriptionError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateDatalakeExceptionsSubscriptionError::unhandled)?;
                    output = output.set_retry_after_seconds(
                        crate::protocol_serde::shape_internal_server_exception::de_retry_after_seconds_header(response.headers())
                                                .map_err(|_|crate::error::CreateDatalakeExceptionsSubscriptionError::unhandled("Failed to parse retryAfterSeconds from header `Retry-After"))?
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
        "ValidationException" => crate::error::CreateDatalakeExceptionsSubscriptionError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateDatalakeExceptionsSubscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreateDatalakeExceptionsSubscriptionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_datalake_exceptions_subscription_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateDatalakeExceptionsSubscriptionOutput, crate::error::CreateDatalakeExceptionsSubscriptionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_datalake_exceptions_subscription_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

