// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_deactivate_user_headers(
                    input: &crate::input::DeactivateUserInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.authentication_token {
        let formatted_2 = inner_1.as_str();
                        if !formatted_2.is_empty() {
                            let header_value = formatted_2;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("authentication_token", format!(
                                "`{}` cannot be used as a header value: {}",
                                &"*** Sensitive Data Redacted ***",
                                err
                            ))
                            })?;
                            builder = builder.header("Authentication", header_value);
                        }
    }
    Ok(builder)
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_deactivate_user_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeactivateUserOutput, crate::error::DeactivateUserError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeactivateUserError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeactivateUserError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "EntityNotExistsException" => crate::error::DeactivateUserError::EntityNotExistsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::entity_not_exists_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_entity_not_exists_exception::de_entity_not_exists_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeactivateUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "FailedDependencyException" => crate::error::DeactivateUserError::FailedDependencyException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::failed_dependency_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_failed_dependency_exception::de_failed_dependency_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeactivateUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceUnavailableException" => crate::error::DeactivateUserError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeactivateUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnauthorizedOperationException" => crate::error::DeactivateUserError::UnauthorizedOperationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unauthorized_operation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unauthorized_operation_exception::de_unauthorized_operation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeactivateUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnauthorizedResourceAccessException" => crate::error::DeactivateUserError::UnauthorizedResourceAccessException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unauthorized_resource_access_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unauthorized_resource_access_exception::de_unauthorized_resource_access_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeactivateUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DeactivateUserError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_deactivate_user_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeactivateUserOutput, crate::error::DeactivateUserError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::deactivate_user_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

