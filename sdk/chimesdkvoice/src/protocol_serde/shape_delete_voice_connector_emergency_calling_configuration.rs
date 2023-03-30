// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_voice_connector_emergency_calling_configuration_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteVoiceConnectorEmergencyCallingConfigurationOutput, crate::error::DeleteVoiceConnectorEmergencyCallingConfigurationError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeleteVoiceConnectorEmergencyCallingConfigurationError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeleteVoiceConnectorEmergencyCallingConfigurationError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::error::DeleteVoiceConnectorEmergencyCallingConfigurationError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteVoiceConnectorEmergencyCallingConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ForbiddenException" => crate::error::DeleteVoiceConnectorEmergencyCallingConfigurationError::ForbiddenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::forbidden_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_forbidden_exception::de_forbidden_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteVoiceConnectorEmergencyCallingConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotFoundException" => crate::error::DeleteVoiceConnectorEmergencyCallingConfigurationError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteVoiceConnectorEmergencyCallingConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceFailureException" => crate::error::DeleteVoiceConnectorEmergencyCallingConfigurationError::ServiceFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_failure_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteVoiceConnectorEmergencyCallingConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceUnavailableException" => crate::error::DeleteVoiceConnectorEmergencyCallingConfigurationError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteVoiceConnectorEmergencyCallingConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ThrottledClientException" => crate::error::DeleteVoiceConnectorEmergencyCallingConfigurationError::ThrottledClientException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttled_client_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_throttled_client_exception::de_throttled_client_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteVoiceConnectorEmergencyCallingConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnauthorizedClientException" => crate::error::DeleteVoiceConnectorEmergencyCallingConfigurationError::UnauthorizedClientException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unauthorized_client_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unauthorized_client_exception::de_unauthorized_client_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteVoiceConnectorEmergencyCallingConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DeleteVoiceConnectorEmergencyCallingConfigurationError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_voice_connector_emergency_calling_configuration_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteVoiceConnectorEmergencyCallingConfigurationOutput, crate::error::DeleteVoiceConnectorEmergencyCallingConfigurationError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_voice_connector_emergency_calling_configuration_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

