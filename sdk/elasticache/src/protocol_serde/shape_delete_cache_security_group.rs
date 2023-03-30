// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_cache_security_group_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteCacheSecurityGroupOutput, crate::error::DeleteCacheSecurityGroupError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeleteCacheSecurityGroupError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeleteCacheSecurityGroupError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "CacheSecurityGroupNotFound" => crate::error::DeleteCacheSecurityGroupError::CacheSecurityGroupNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cache_security_group_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cache_security_group_not_found_fault::de_cache_security_group_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::DeleteCacheSecurityGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidCacheSecurityGroupState" => crate::error::DeleteCacheSecurityGroupError::InvalidCacheSecurityGroupStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_cache_security_group_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_cache_security_group_state_fault::de_invalid_cache_security_group_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::DeleteCacheSecurityGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterCombination" => crate::error::DeleteCacheSecurityGroupError::InvalidParameterCombinationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_combination_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_combination_exception::de_invalid_parameter_combination_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DeleteCacheSecurityGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterValue" => crate::error::DeleteCacheSecurityGroupError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_value_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DeleteCacheSecurityGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DeleteCacheSecurityGroupError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_cache_security_group_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteCacheSecurityGroupOutput, crate::error::DeleteCacheSecurityGroupError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_cache_security_group_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

