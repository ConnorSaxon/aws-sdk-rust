// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_stack_set_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteStackSetOutput, crate::error::DeleteStackSetError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeleteStackSetError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeleteStackSetError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "OperationInProgressException" => crate::error::DeleteStackSetError::OperationInProgressException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::operation_in_progress_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_in_progress_exception::de_operation_in_progress_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DeleteStackSetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "StackSetNotEmptyException" => crate::error::DeleteStackSetError::StackSetNotEmptyException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::stack_set_not_empty_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_stack_set_not_empty_exception::de_stack_set_not_empty_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DeleteStackSetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DeleteStackSetError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_stack_set_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteStackSetOutput, crate::error::DeleteStackSetError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_stack_set_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

