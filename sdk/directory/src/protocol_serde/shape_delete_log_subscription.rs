// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_log_subscription_input(input: &crate::input::DeleteLogSubscriptionInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_log_subscription_input::ser_delete_log_subscription_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_log_subscription_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteLogSubscriptionOutput, crate::error::DeleteLogSubscriptionError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeleteLogSubscriptionError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeleteLogSubscriptionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ClientException" => crate::error::DeleteLogSubscriptionError::ClientException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::client_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_client_exception::de_client_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteLogSubscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EntityDoesNotExistException" => crate::error::DeleteLogSubscriptionError::EntityDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::entity_does_not_exist_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_entity_does_not_exist_exception::de_entity_does_not_exist_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteLogSubscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceException" => crate::error::DeleteLogSubscriptionError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteLogSubscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedOperationException" => crate::error::DeleteLogSubscriptionError::UnsupportedOperationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unsupported_operation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteLogSubscriptionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DeleteLogSubscriptionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_log_subscription_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteLogSubscriptionOutput, crate::error::DeleteLogSubscriptionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_log_subscription_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

