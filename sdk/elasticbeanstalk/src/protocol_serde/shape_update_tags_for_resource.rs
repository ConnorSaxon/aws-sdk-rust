// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_update_tags_for_resource_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateTagsForResourceOutput, crate::error::UpdateTagsForResourceError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::UpdateTagsForResourceError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UpdateTagsForResourceError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InsufficientPrivilegesException" => crate::error::UpdateTagsForResourceError::InsufficientPrivilegesException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::insufficient_privileges_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_insufficient_privileges_exception::de_insufficient_privileges_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateTagsForResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OperationInProgressFailure" => crate::error::UpdateTagsForResourceError::OperationInProgressException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::operation_in_progress_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_in_progress_exception::de_operation_in_progress_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateTagsForResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::UpdateTagsForResourceError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateTagsForResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceTypeNotSupportedException" => crate::error::UpdateTagsForResourceError::ResourceTypeNotSupportedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_type_not_supported_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_type_not_supported_exception::de_resource_type_not_supported_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateTagsForResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyTagsException" => crate::error::UpdateTagsForResourceError::TooManyTagsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_tags_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_tags_exception::de_too_many_tags_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::UpdateTagsForResourceError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::UpdateTagsForResourceError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_tags_for_resource_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateTagsForResourceOutput, crate::error::UpdateTagsForResourceError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::update_tags_for_resource_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

