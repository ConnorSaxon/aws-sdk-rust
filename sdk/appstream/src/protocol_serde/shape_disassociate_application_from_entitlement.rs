// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disassociate_application_from_entitlement_input(input: &crate::input::DisassociateApplicationFromEntitlementInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_disassociate_application_from_entitlement_input::ser_disassociate_application_from_entitlement_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_disassociate_application_from_entitlement_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DisassociateApplicationFromEntitlementOutput, crate::error::DisassociateApplicationFromEntitlementError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DisassociateApplicationFromEntitlementError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DisassociateApplicationFromEntitlementError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "EntitlementNotFoundException" => crate::error::DisassociateApplicationFromEntitlementError::EntitlementNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::entitlement_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_entitlement_not_found_exception::de_entitlement_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DisassociateApplicationFromEntitlementError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OperationNotPermittedException" => crate::error::DisassociateApplicationFromEntitlementError::OperationNotPermittedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::operation_not_permitted_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_not_permitted_exception::de_operation_not_permitted_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DisassociateApplicationFromEntitlementError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::DisassociateApplicationFromEntitlementError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DisassociateApplicationFromEntitlementError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DisassociateApplicationFromEntitlementError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_disassociate_application_from_entitlement_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DisassociateApplicationFromEntitlementOutput, crate::error::DisassociateApplicationFromEntitlementError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::disassociate_application_from_entitlement_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

