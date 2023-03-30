// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_resource_policy_input(_input: &crate::input::DeleteResourcePolicyInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    Ok(aws_smithy_http::body::SdkBody::from("{}"))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_resource_policy_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteResourcePolicyOutput, crate::error::DeleteResourcePolicyError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeleteResourcePolicyError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeleteResourcePolicyError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::DeleteResourcePolicyError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteResourcePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "AWSOrganizationsNotInUseException" => crate::error::DeleteResourcePolicyError::AwsOrganizationsNotInUseException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::aws_organizations_not_in_use_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_aws_organizations_not_in_use_exception::de_aws_organizations_not_in_use_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteResourcePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ConcurrentModificationException" => crate::error::DeleteResourcePolicyError::ConcurrentModificationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::concurrent_modification_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_concurrent_modification_exception::de_concurrent_modification_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteResourcePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ConstraintViolationException" => crate::error::DeleteResourcePolicyError::ConstraintViolationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::constraint_violation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_constraint_violation_exception::de_constraint_violation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteResourcePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourcePolicyNotFoundException" => crate::error::DeleteResourcePolicyError::ResourcePolicyNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_policy_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_policy_not_found_exception::de_resource_policy_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteResourcePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceException" => crate::error::DeleteResourcePolicyError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteResourcePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRequestsException" => crate::error::DeleteResourcePolicyError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_requests_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteResourcePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedAPIEndpointException" => crate::error::DeleteResourcePolicyError::UnsupportedApiEndpointException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unsupported_api_endpoint_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_api_endpoint_exception::de_unsupported_api_endpoint_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteResourcePolicyError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DeleteResourcePolicyError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_resource_policy_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteResourcePolicyOutput, crate::error::DeleteResourcePolicyError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_resource_policy_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

