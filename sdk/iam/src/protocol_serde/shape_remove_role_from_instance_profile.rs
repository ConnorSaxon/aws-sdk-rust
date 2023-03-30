// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_remove_role_from_instance_profile_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::RemoveRoleFromInstanceProfileOutput, crate::error::RemoveRoleFromInstanceProfileError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::RemoveRoleFromInstanceProfileError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::RemoveRoleFromInstanceProfileError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "LimitExceeded" => crate::error::RemoveRoleFromInstanceProfileError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::RemoveRoleFromInstanceProfileError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchEntity" => crate::error::RemoveRoleFromInstanceProfileError::NoSuchEntityException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_such_entity_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_entity_exception::de_no_such_entity_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::RemoveRoleFromInstanceProfileError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceFailure" => crate::error::RemoveRoleFromInstanceProfileError::ServiceFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_failure_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_failure_exception::de_service_failure_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::RemoveRoleFromInstanceProfileError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnmodifiableEntity" => crate::error::RemoveRoleFromInstanceProfileError::UnmodifiableEntityException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unmodifiable_entity_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unmodifiable_entity_exception::de_unmodifiable_entity_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::RemoveRoleFromInstanceProfileError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::RemoveRoleFromInstanceProfileError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_remove_role_from_instance_profile_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::RemoveRoleFromInstanceProfileOutput, crate::error::RemoveRoleFromInstanceProfileError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::remove_role_from_instance_profile_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

