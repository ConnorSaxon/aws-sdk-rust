// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_distributions_by_cache_policy_id_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListDistributionsByCachePolicyIdOutput, crate::error::ListDistributionsByCachePolicyIdError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ListDistributionsByCachePolicyIdError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ListDistributionsByCachePolicyIdError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDenied" => crate::error::ListDistributionsByCachePolicyIdError::AccessDenied({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied::de_access_denied_xml_err(response.body().as_ref(), output).map_err(crate::error::ListDistributionsByCachePolicyIdError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidArgument" => crate::error::ListDistributionsByCachePolicyIdError::InvalidArgument({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_argument::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_argument::de_invalid_argument_xml_err(response.body().as_ref(), output).map_err(crate::error::ListDistributionsByCachePolicyIdError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchCachePolicy" => crate::error::ListDistributionsByCachePolicyIdError::NoSuchCachePolicy({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_such_cache_policy::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_cache_policy::de_no_such_cache_policy_xml_err(response.body().as_ref(), output).map_err(crate::error::ListDistributionsByCachePolicyIdError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ListDistributionsByCachePolicyIdError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_distributions_by_cache_policy_id_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListDistributionsByCachePolicyIdOutput, crate::error::ListDistributionsByCachePolicyIdError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_distributions_by_cache_policy_id_output::Builder::default();
        let _ = response;
        output = output.set_distribution_id_list(
            crate::protocol_serde::shape_list_distributions_by_cache_policy_id_output::de_distribution_id_list_payload(response.body().as_ref())?
        );
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

