// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_remove_endpoints_input(input: &crate::input::RemoveEndpointsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_remove_endpoints_input::ser_remove_endpoints_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_remove_endpoints_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::RemoveEndpointsOutput, crate::error::RemoveEndpointsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::RemoveEndpointsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::RemoveEndpointsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::RemoveEndpointsError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::RemoveEndpointsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EndpointGroupNotFoundException" => crate::error::RemoveEndpointsError::EndpointGroupNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::endpoint_group_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_endpoint_group_not_found_exception::de_endpoint_group_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::RemoveEndpointsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServiceErrorException" => crate::error::RemoveEndpointsError::InternalServiceErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_error_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_service_error_exception::de_internal_service_error_exception_json_err(response.body().as_ref(), output).map_err(crate::error::RemoveEndpointsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidArgumentException" => crate::error::RemoveEndpointsError::InvalidArgumentException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_argument_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_argument_exception::de_invalid_argument_exception_json_err(response.body().as_ref(), output).map_err(crate::error::RemoveEndpointsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TransactionInProgressException" => crate::error::RemoveEndpointsError::TransactionInProgressException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::transaction_in_progress_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_transaction_in_progress_exception::de_transaction_in_progress_exception_json_err(response.body().as_ref(), output).map_err(crate::error::RemoveEndpointsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::RemoveEndpointsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_remove_endpoints_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::RemoveEndpointsOutput, crate::error::RemoveEndpointsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::remove_endpoints_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

