// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_lb_cookie_stickiness_policy_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateLbCookieStickinessPolicyOutput, crate::error::CreateLBCookieStickinessPolicyError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateLBCookieStickinessPolicyError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateLBCookieStickinessPolicyError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "LoadBalancerNotFound" => crate::error::CreateLBCookieStickinessPolicyError::AccessPointNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_point_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_point_not_found_exception::de_access_point_not_found_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateLBCookieStickinessPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DuplicatePolicyName" => crate::error::CreateLBCookieStickinessPolicyError::DuplicatePolicyNameException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::duplicate_policy_name_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_duplicate_policy_name_exception::de_duplicate_policy_name_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateLBCookieStickinessPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidConfigurationRequest" => crate::error::CreateLBCookieStickinessPolicyError::InvalidConfigurationRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_configuration_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_configuration_request_exception::de_invalid_configuration_request_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateLBCookieStickinessPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyPolicies" => crate::error::CreateLBCookieStickinessPolicyError::TooManyPoliciesException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_policies_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_policies_exception::de_too_many_policies_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateLBCookieStickinessPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreateLBCookieStickinessPolicyError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_lb_cookie_stickiness_policy_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateLbCookieStickinessPolicyOutput, crate::error::CreateLBCookieStickinessPolicyError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_lb_cookie_stickiness_policy_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

