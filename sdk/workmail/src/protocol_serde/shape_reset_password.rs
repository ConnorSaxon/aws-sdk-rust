// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_reset_password_input(input: &crate::input::ResetPasswordInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_reset_password_input::ser_reset_password_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_reset_password_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ResetPasswordOutput, crate::error::ResetPasswordError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ResetPasswordError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ResetPasswordError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "DirectoryServiceAuthenticationFailedException" => crate::error::ResetPasswordError::DirectoryServiceAuthenticationFailedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::directory_service_authentication_failed_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_directory_service_authentication_failed_exception::de_directory_service_authentication_failed_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ResetPasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DirectoryUnavailableException" => crate::error::ResetPasswordError::DirectoryUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::directory_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_directory_unavailable_exception::de_directory_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ResetPasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EntityNotFoundException" => crate::error::ResetPasswordError::EntityNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::entity_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_entity_not_found_exception::de_entity_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ResetPasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EntityStateException" => crate::error::ResetPasswordError::EntityStateException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::entity_state_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_entity_state_exception::de_entity_state_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ResetPasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterException" => crate::error::ResetPasswordError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ResetPasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidPasswordException" => crate::error::ResetPasswordError::InvalidPasswordException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_password_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_password_exception::de_invalid_password_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ResetPasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OrganizationNotFoundException" => crate::error::ResetPasswordError::OrganizationNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::organization_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_organization_not_found_exception::de_organization_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ResetPasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OrganizationStateException" => crate::error::ResetPasswordError::OrganizationStateException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::organization_state_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_organization_state_exception::de_organization_state_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ResetPasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedOperationException" => crate::error::ResetPasswordError::UnsupportedOperationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unsupported_operation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ResetPasswordError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ResetPasswordError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_reset_password_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ResetPasswordOutput, crate::error::ResetPasswordError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::reset_password_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

