// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disassociate_admin_account_input(_input: &crate::input::DisassociateAdminAccountInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    Ok(aws_smithy_http::body::SdkBody::from("{}"))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_disassociate_admin_account_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DisassociateAdminAccountOutput, crate::error::DisassociateAdminAccountError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DisassociateAdminAccountError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DisassociateAdminAccountError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InternalErrorException" => crate::error::DisassociateAdminAccountError::InternalErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_error_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_error_exception::de_internal_error_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DisassociateAdminAccountError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidOperationException" => crate::error::DisassociateAdminAccountError::InvalidOperationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_operation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_operation_exception::de_invalid_operation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DisassociateAdminAccountError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::DisassociateAdminAccountError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DisassociateAdminAccountError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DisassociateAdminAccountError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_disassociate_admin_account_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DisassociateAdminAccountOutput, crate::error::DisassociateAdminAccountError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::disassociate_admin_account_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

