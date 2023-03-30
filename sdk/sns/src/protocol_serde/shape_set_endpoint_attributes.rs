// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_set_endpoint_attributes_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::SetEndpointAttributesOutput, crate::error::SetEndpointAttributesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::SetEndpointAttributesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::SetEndpointAttributesError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AuthorizationError" => crate::error::SetEndpointAttributesError::AuthorizationErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::authorization_error_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_authorization_error_exception::de_authorization_error_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::SetEndpointAttributesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalError" => crate::error::SetEndpointAttributesError::InternalErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_error_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_error_exception::de_internal_error_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::SetEndpointAttributesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameter" => crate::error::SetEndpointAttributesError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::SetEndpointAttributesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotFound" => crate::error::SetEndpointAttributesError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::SetEndpointAttributesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::SetEndpointAttributesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_set_endpoint_attributes_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::SetEndpointAttributesOutput, crate::error::SetEndpointAttributesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::set_endpoint_attributes_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

