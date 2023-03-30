// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_deny_custom_routing_traffic_input(input: &crate::input::DenyCustomRoutingTrafficInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_deny_custom_routing_traffic_input::ser_deny_custom_routing_traffic_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_deny_custom_routing_traffic_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DenyCustomRoutingTrafficOutput, crate::error::DenyCustomRoutingTrafficError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DenyCustomRoutingTrafficError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DenyCustomRoutingTrafficError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "EndpointGroupNotFoundException" => crate::error::DenyCustomRoutingTrafficError::EndpointGroupNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::endpoint_group_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_endpoint_group_not_found_exception::de_endpoint_group_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DenyCustomRoutingTrafficError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServiceErrorException" => crate::error::DenyCustomRoutingTrafficError::InternalServiceErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_error_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_service_error_exception::de_internal_service_error_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DenyCustomRoutingTrafficError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidArgumentException" => crate::error::DenyCustomRoutingTrafficError::InvalidArgumentException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_argument_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_argument_exception::de_invalid_argument_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DenyCustomRoutingTrafficError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DenyCustomRoutingTrafficError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_deny_custom_routing_traffic_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DenyCustomRoutingTrafficOutput, crate::error::DenyCustomRoutingTrafficError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::deny_custom_routing_traffic_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

