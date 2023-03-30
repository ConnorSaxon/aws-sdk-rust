// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_register_workflow_type_input(input: &crate::input::RegisterWorkflowTypeInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_register_workflow_type_input::ser_register_workflow_type_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_register_workflow_type_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::RegisterWorkflowTypeOutput, crate::error::RegisterWorkflowTypeError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::RegisterWorkflowTypeError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::RegisterWorkflowTypeError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "LimitExceededFault" => crate::error::RegisterWorkflowTypeError::LimitExceededFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_fault::de_limit_exceeded_fault_json_err(response.body().as_ref(), output).map_err(crate::error::RegisterWorkflowTypeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OperationNotPermittedFault" => crate::error::RegisterWorkflowTypeError::OperationNotPermittedFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::operation_not_permitted_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_not_permitted_fault::de_operation_not_permitted_fault_json_err(response.body().as_ref(), output).map_err(crate::error::RegisterWorkflowTypeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TypeAlreadyExistsFault" => crate::error::RegisterWorkflowTypeError::TypeAlreadyExistsFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::type_already_exists_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_type_already_exists_fault::de_type_already_exists_fault_json_err(response.body().as_ref(), output).map_err(crate::error::RegisterWorkflowTypeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnknownResourceFault" => crate::error::RegisterWorkflowTypeError::UnknownResourceFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unknown_resource_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unknown_resource_fault::de_unknown_resource_fault_json_err(response.body().as_ref(), output).map_err(crate::error::RegisterWorkflowTypeError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::RegisterWorkflowTypeError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_register_workflow_type_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::RegisterWorkflowTypeOutput, crate::error::RegisterWorkflowTypeError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::register_workflow_type_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

