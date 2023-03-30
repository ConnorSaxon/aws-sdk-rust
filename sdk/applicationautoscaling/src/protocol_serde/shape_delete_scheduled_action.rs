// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_scheduled_action_input(input: &crate::input::DeleteScheduledActionInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_scheduled_action_input::ser_delete_scheduled_action_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_scheduled_action_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteScheduledActionOutput, crate::error::DeleteScheduledActionError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeleteScheduledActionError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeleteScheduledActionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ConcurrentUpdateException" => crate::error::DeleteScheduledActionError::ConcurrentUpdateException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::concurrent_update_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_concurrent_update_exception::de_concurrent_update_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteScheduledActionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServiceException" => crate::error::DeleteScheduledActionError::InternalServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_service_exception::de_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteScheduledActionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ObjectNotFoundException" => crate::error::DeleteScheduledActionError::ObjectNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::object_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_object_not_found_exception::de_object_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteScheduledActionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ValidationException" => crate::error::DeleteScheduledActionError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteScheduledActionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DeleteScheduledActionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_scheduled_action_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteScheduledActionOutput, crate::error::DeleteScheduledActionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_scheduled_action_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

