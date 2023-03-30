// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_set_load_balancer_listener_ssl_certificate_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::SetLoadBalancerListenerSslCertificateOutput, crate::error::SetLoadBalancerListenerSSLCertificateError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::SetLoadBalancerListenerSSLCertificateError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::SetLoadBalancerListenerSSLCertificateError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "LoadBalancerNotFound" => crate::error::SetLoadBalancerListenerSSLCertificateError::AccessPointNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_point_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_point_not_found_exception::de_access_point_not_found_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::SetLoadBalancerListenerSSLCertificateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "CertificateNotFound" => crate::error::SetLoadBalancerListenerSSLCertificateError::CertificateNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::certificate_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_certificate_not_found_exception::de_certificate_not_found_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::SetLoadBalancerListenerSSLCertificateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidConfigurationRequest" => crate::error::SetLoadBalancerListenerSSLCertificateError::InvalidConfigurationRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_configuration_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_configuration_request_exception::de_invalid_configuration_request_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::SetLoadBalancerListenerSSLCertificateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ListenerNotFound" => crate::error::SetLoadBalancerListenerSSLCertificateError::ListenerNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::listener_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_listener_not_found_exception::de_listener_not_found_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::SetLoadBalancerListenerSSLCertificateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedProtocol" => crate::error::SetLoadBalancerListenerSSLCertificateError::UnsupportedProtocolException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unsupported_protocol_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_protocol_exception::de_unsupported_protocol_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::SetLoadBalancerListenerSSLCertificateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::SetLoadBalancerListenerSSLCertificateError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_set_load_balancer_listener_ssl_certificate_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::SetLoadBalancerListenerSslCertificateOutput, crate::error::SetLoadBalancerListenerSSLCertificateError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::set_load_balancer_listener_ssl_certificate_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

