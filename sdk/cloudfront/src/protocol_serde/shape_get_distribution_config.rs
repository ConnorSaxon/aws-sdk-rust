// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_distribution_config_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetDistributionConfigOutput, crate::error::GetDistributionConfigError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetDistributionConfigError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetDistributionConfigError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDenied" => crate::error::GetDistributionConfigError::AccessDenied({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied::de_access_denied_xml_err(response.body().as_ref(), output).map_err(crate::error::GetDistributionConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchDistribution" => crate::error::GetDistributionConfigError::NoSuchDistribution({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_such_distribution::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_distribution::de_no_such_distribution_xml_err(response.body().as_ref(), output).map_err(crate::error::GetDistributionConfigError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GetDistributionConfigError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_distribution_config_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetDistributionConfigOutput, crate::error::GetDistributionConfigError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_distribution_config_output::Builder::default();
        let _ = response;
        output = output.set_distribution_config(
            crate::protocol_serde::shape_get_distribution_config_output::de_distribution_config_payload(response.body().as_ref())?
        );
        output = output.set_e_tag(
            crate::protocol_serde::shape_get_distribution_config_output::de_e_tag_header(response.headers())
                                    .map_err(|_|crate::error::GetDistributionConfigError::unhandled("Failed to parse ETag from header `ETag"))?
        );
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

