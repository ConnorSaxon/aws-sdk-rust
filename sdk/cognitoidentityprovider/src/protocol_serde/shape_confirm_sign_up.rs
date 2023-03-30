// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_confirm_sign_up_input(input: &crate::input::ConfirmSignUpInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_confirm_sign_up_input::ser_confirm_sign_up_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_confirm_sign_up_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ConfirmSignUpOutput, crate::error::ConfirmSignUpError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ConfirmSignUpError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ConfirmSignUpError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AliasExistsException" => crate::error::ConfirmSignUpError::AliasExistsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::alias_exists_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_alias_exists_exception::de_alias_exists_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ConfirmSignUpError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CodeMismatchException" => crate::error::ConfirmSignUpError::CodeMismatchException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::code_mismatch_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_code_mismatch_exception::de_code_mismatch_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ConfirmSignUpError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ExpiredCodeException" => crate::error::ConfirmSignUpError::ExpiredCodeException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::expired_code_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_expired_code_exception::de_expired_code_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ConfirmSignUpError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ForbiddenException" => crate::error::ConfirmSignUpError::ForbiddenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::forbidden_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_forbidden_exception::de_forbidden_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ConfirmSignUpError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalErrorException" => crate::error::ConfirmSignUpError::InternalErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_error_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_error_exception::de_internal_error_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ConfirmSignUpError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidLambdaResponseException" => crate::error::ConfirmSignUpError::InvalidLambdaResponseException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_lambda_response_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_lambda_response_exception::de_invalid_lambda_response_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ConfirmSignUpError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterException" => crate::error::ConfirmSignUpError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ConfirmSignUpError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceededException" => crate::error::ConfirmSignUpError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ConfirmSignUpError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotAuthorizedException" => crate::error::ConfirmSignUpError::NotAuthorizedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::not_authorized_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_authorized_exception::de_not_authorized_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ConfirmSignUpError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::ConfirmSignUpError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ConfirmSignUpError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyFailedAttemptsException" => crate::error::ConfirmSignUpError::TooManyFailedAttemptsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_failed_attempts_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_failed_attempts_exception::de_too_many_failed_attempts_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ConfirmSignUpError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRequestsException" => crate::error::ConfirmSignUpError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_requests_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ConfirmSignUpError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnexpectedLambdaException" => crate::error::ConfirmSignUpError::UnexpectedLambdaException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unexpected_lambda_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unexpected_lambda_exception::de_unexpected_lambda_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ConfirmSignUpError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UserLambdaValidationException" => crate::error::ConfirmSignUpError::UserLambdaValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::user_lambda_validation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_user_lambda_validation_exception::de_user_lambda_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ConfirmSignUpError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UserNotFoundException" => crate::error::ConfirmSignUpError::UserNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::user_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_user_not_found_exception::de_user_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ConfirmSignUpError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ConfirmSignUpError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_confirm_sign_up_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ConfirmSignUpOutput, crate::error::ConfirmSignUpError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::confirm_sign_up_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

