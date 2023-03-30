// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_cluster_security_group_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteClusterSecurityGroupOutput, crate::error::DeleteClusterSecurityGroupError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeleteClusterSecurityGroupError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeleteClusterSecurityGroupError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ClusterSecurityGroupNotFound" => crate::error::DeleteClusterSecurityGroupError::ClusterSecurityGroupNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cluster_security_group_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cluster_security_group_not_found_fault::de_cluster_security_group_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::DeleteClusterSecurityGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidClusterSecurityGroupState" => crate::error::DeleteClusterSecurityGroupError::InvalidClusterSecurityGroupStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_cluster_security_group_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_cluster_security_group_state_fault::de_invalid_cluster_security_group_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::DeleteClusterSecurityGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DeleteClusterSecurityGroupError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_cluster_security_group_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteClusterSecurityGroupOutput, crate::error::DeleteClusterSecurityGroupError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_cluster_security_group_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

