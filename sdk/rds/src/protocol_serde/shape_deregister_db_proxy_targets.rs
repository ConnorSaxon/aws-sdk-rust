// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_deregister_db_proxy_targets_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeregisterDbProxyTargetsOutput, crate::error::DeregisterDBProxyTargetsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeregisterDBProxyTargetsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeregisterDBProxyTargetsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "DBProxyNotFoundFault" => crate::error::DeregisterDBProxyTargetsError::DbProxyNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::db_proxy_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_proxy_not_found_fault::de_db_proxy_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::DeregisterDBProxyTargetsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DBProxyTargetGroupNotFoundFault" => crate::error::DeregisterDBProxyTargetsError::DbProxyTargetGroupNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::db_proxy_target_group_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_proxy_target_group_not_found_fault::de_db_proxy_target_group_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::DeregisterDBProxyTargetsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DBProxyTargetNotFoundFault" => crate::error::DeregisterDBProxyTargetsError::DbProxyTargetNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::db_proxy_target_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_proxy_target_not_found_fault::de_db_proxy_target_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::DeregisterDBProxyTargetsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidDBProxyStateFault" => crate::error::DeregisterDBProxyTargetsError::InvalidDbProxyStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_db_proxy_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_db_proxy_state_fault::de_invalid_db_proxy_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::DeregisterDBProxyTargetsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DeregisterDBProxyTargetsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_deregister_db_proxy_targets_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeregisterDbProxyTargetsOutput, crate::error::DeregisterDBProxyTargetsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::deregister_db_proxy_targets_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

