// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_origin_access_control_headers(
                    input: &crate::input::UpdateOriginAccessControlInput,
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
pub fn de_update_origin_access_control_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateOriginAccessControlOutput, crate::error::UpdateOriginAccessControlError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::UpdateOriginAccessControlError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UpdateOriginAccessControlError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDenied" => crate::error::UpdateOriginAccessControlError::AccessDenied({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied::de_access_denied_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateOriginAccessControlError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "IllegalUpdate" => crate::error::UpdateOriginAccessControlError::IllegalUpdate({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::illegal_update::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_illegal_update::de_illegal_update_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateOriginAccessControlError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidArgument" => crate::error::UpdateOriginAccessControlError::InvalidArgument({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_argument::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_argument::de_invalid_argument_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateOriginAccessControlError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidIfMatchVersion" => crate::error::UpdateOriginAccessControlError::InvalidIfMatchVersion({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_if_match_version::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_if_match_version::de_invalid_if_match_version_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateOriginAccessControlError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchOriginAccessControl" => crate::error::UpdateOriginAccessControlError::NoSuchOriginAccessControl({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_such_origin_access_control::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_origin_access_control::de_no_such_origin_access_control_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateOriginAccessControlError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OriginAccessControlAlreadyExists" => crate::error::UpdateOriginAccessControlError::OriginAccessControlAlreadyExists({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::origin_access_control_already_exists::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_origin_access_control_already_exists::de_origin_access_control_already_exists_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateOriginAccessControlError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "PreconditionFailed" => crate::error::UpdateOriginAccessControlError::PreconditionFailed({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::precondition_failed::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_precondition_failed::de_precondition_failed_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateOriginAccessControlError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::UpdateOriginAccessControlError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_origin_access_control_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateOriginAccessControlOutput, crate::error::UpdateOriginAccessControlError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::update_origin_access_control_output::Builder::default();
        let _ = response;
        output = output.set_e_tag(
            crate::protocol_serde::shape_update_origin_access_control_output::de_e_tag_header(response.headers())
                                    .map_err(|_|crate::error::UpdateOriginAccessControlError::unhandled("Failed to parse ETag from header `ETag"))?
        );
        output = output.set_origin_access_control(
            crate::protocol_serde::shape_update_origin_access_control_output::de_origin_access_control_payload(response.body().as_ref())?
        );
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

