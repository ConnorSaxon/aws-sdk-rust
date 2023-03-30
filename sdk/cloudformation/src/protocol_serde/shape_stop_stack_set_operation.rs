// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_stop_stack_set_operation_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StopStackSetOperationOutput, crate::error::StopStackSetOperationError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::StopStackSetOperationError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::StopStackSetOperationError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidOperationException" => crate::error::StopStackSetOperationError::InvalidOperationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_operation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_operation_exception::de_invalid_operation_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::StopStackSetOperationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OperationNotFoundException" => crate::error::StopStackSetOperationError::OperationNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::operation_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_not_found_exception::de_operation_not_found_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::StopStackSetOperationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "StackSetNotFoundException" => crate::error::StopStackSetOperationError::StackSetNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::stack_set_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_stack_set_not_found_exception::de_stack_set_not_found_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::StopStackSetOperationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::StopStackSetOperationError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_stop_stack_set_operation_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StopStackSetOperationOutput, crate::error::StopStackSetOperationError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::stop_stack_set_operation_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

