// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_associate_web_acl_input(input: &crate::input::AssociateWebAclInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_associate_web_acl_input::ser_associate_web_acl_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_web_acl_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AssociateWebAclOutput, crate::error::AssociateWebACLError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::AssociateWebACLError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::AssociateWebACLError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "WAFInternalErrorException" => crate::error::AssociateWebACLError::WafInternalErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_internal_error_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_internal_error_exception::de_waf_internal_error_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AssociateWebACLError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFInvalidOperationException" => crate::error::AssociateWebACLError::WafInvalidOperationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_invalid_operation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_invalid_operation_exception::de_waf_invalid_operation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AssociateWebACLError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFInvalidParameterException" => crate::error::AssociateWebACLError::WafInvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_invalid_parameter_exception::de_waf_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AssociateWebACLError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFNonexistentItemException" => crate::error::AssociateWebACLError::WafNonexistentItemException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_nonexistent_item_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_nonexistent_item_exception::de_waf_nonexistent_item_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AssociateWebACLError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFUnavailableEntityException" => crate::error::AssociateWebACLError::WafUnavailableEntityException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_unavailable_entity_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_unavailable_entity_exception::de_waf_unavailable_entity_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AssociateWebACLError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::AssociateWebACLError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_web_acl_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AssociateWebAclOutput, crate::error::AssociateWebACLError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::associate_web_acl_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

