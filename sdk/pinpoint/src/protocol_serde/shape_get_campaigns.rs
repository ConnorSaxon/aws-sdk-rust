// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_campaigns_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetCampaignsOutput, crate::error::GetCampaignsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetCampaignsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetCampaignsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::error::GetCampaignsError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetCampaignsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ForbiddenException" => crate::error::GetCampaignsError::ForbiddenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::forbidden_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_forbidden_exception::de_forbidden_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetCampaignsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerErrorException" => crate::error::GetCampaignsError::InternalServerErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error_exception::de_internal_server_error_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetCampaignsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "MethodNotAllowedException" => crate::error::GetCampaignsError::MethodNotAllowedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::method_not_allowed_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_method_not_allowed_exception::de_method_not_allowed_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetCampaignsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotFoundException" => crate::error::GetCampaignsError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetCampaignsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "PayloadTooLargeException" => crate::error::GetCampaignsError::PayloadTooLargeException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::payload_too_large_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_payload_too_large_exception::de_payload_too_large_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetCampaignsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRequestsException" => crate::error::GetCampaignsError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_requests_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetCampaignsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GetCampaignsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_campaigns_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetCampaignsOutput, crate::error::GetCampaignsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_campaigns_output::Builder::default();
        let _ = response;
        output = output.set_campaigns_response(
            crate::protocol_serde::shape_get_campaigns_output::de_campaigns_response_payload(response.body().as_ref())?
        );
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

