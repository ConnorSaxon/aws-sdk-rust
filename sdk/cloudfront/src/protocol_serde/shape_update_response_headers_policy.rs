// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_response_headers_policy_headers(
                    input: &crate::input::UpdateResponseHeadersPolicyInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.if_match {
        let formatted_2 = inner_1.as_str();
                        if !formatted_2.is_empty() {
                            let header_value = formatted_2;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("if_match", format!(
                                "`{}` cannot be used as a header value: {}",
                                &header_value,
                                err
                            ))
                            })?;
                            builder = builder.header("If-Match", header_value);
                        }
    }
    Ok(builder)
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_response_headers_policy_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateResponseHeadersPolicyOutput, crate::error::UpdateResponseHeadersPolicyError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::UpdateResponseHeadersPolicyError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UpdateResponseHeadersPolicyError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDenied" => crate::error::UpdateResponseHeadersPolicyError::AccessDenied({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied::de_access_denied_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateResponseHeadersPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "IllegalUpdate" => crate::error::UpdateResponseHeadersPolicyError::IllegalUpdate({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::illegal_update::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_illegal_update::de_illegal_update_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateResponseHeadersPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InconsistentQuantities" => crate::error::UpdateResponseHeadersPolicyError::InconsistentQuantities({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::inconsistent_quantities::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_inconsistent_quantities::de_inconsistent_quantities_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateResponseHeadersPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidArgument" => crate::error::UpdateResponseHeadersPolicyError::InvalidArgument({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_argument::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_argument::de_invalid_argument_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateResponseHeadersPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidIfMatchVersion" => crate::error::UpdateResponseHeadersPolicyError::InvalidIfMatchVersion({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_if_match_version::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_if_match_version::de_invalid_if_match_version_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateResponseHeadersPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchResponseHeadersPolicy" => crate::error::UpdateResponseHeadersPolicyError::NoSuchResponseHeadersPolicy({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_such_response_headers_policy::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_response_headers_policy::de_no_such_response_headers_policy_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateResponseHeadersPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "PreconditionFailed" => crate::error::UpdateResponseHeadersPolicyError::PreconditionFailed({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::precondition_failed::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_precondition_failed::de_precondition_failed_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateResponseHeadersPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResponseHeadersPolicyAlreadyExists" => crate::error::UpdateResponseHeadersPolicyError::ResponseHeadersPolicyAlreadyExists({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::response_headers_policy_already_exists::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_response_headers_policy_already_exists::de_response_headers_policy_already_exists_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateResponseHeadersPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooLongCSPInResponseHeadersPolicy" => crate::error::UpdateResponseHeadersPolicyError::TooLongCspInResponseHeadersPolicy({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_long_csp_in_response_headers_policy::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_long_csp_in_response_headers_policy::de_too_long_csp_in_response_headers_policy_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateResponseHeadersPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyCustomHeadersInResponseHeadersPolicy" => crate::error::UpdateResponseHeadersPolicyError::TooManyCustomHeadersInResponseHeadersPolicy({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_custom_headers_in_response_headers_policy::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_custom_headers_in_response_headers_policy::de_too_many_custom_headers_in_response_headers_policy_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateResponseHeadersPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRemoveHeadersInResponseHeadersPolicy" => crate::error::UpdateResponseHeadersPolicyError::TooManyRemoveHeadersInResponseHeadersPolicy({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_remove_headers_in_response_headers_policy::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_remove_headers_in_response_headers_policy::de_too_many_remove_headers_in_response_headers_policy_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateResponseHeadersPolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::UpdateResponseHeadersPolicyError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_response_headers_policy_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateResponseHeadersPolicyOutput, crate::error::UpdateResponseHeadersPolicyError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::update_response_headers_policy_output::Builder::default();
        let _ = response;
        output = output.set_e_tag(
            crate::protocol_serde::shape_update_response_headers_policy_output::de_e_tag_header(response.headers())
                                    .map_err(|_|crate::error::UpdateResponseHeadersPolicyError::unhandled("Failed to parse ETag from header `ETag"))?
        );
        output = output.set_response_headers_policy(
            crate::protocol_serde::shape_update_response_headers_policy_output::de_response_headers_policy_payload(response.body().as_ref())?
        );
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

