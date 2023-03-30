// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_service_quota_increase_request_from_template_input(input: &crate::input::DeleteServiceQuotaIncreaseRequestFromTemplateInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_service_quota_increase_request_from_template_input::ser_delete_service_quota_increase_request_from_template_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_service_quota_increase_request_from_template_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteServiceQuotaIncreaseRequestFromTemplateOutput, crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "AWSServiceAccessNotEnabledException" => crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError::AwsServiceAccessNotEnabledException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::aws_service_access_not_enabled_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_aws_service_access_not_enabled_exception::de_aws_service_access_not_enabled_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DependencyAccessDeniedException" => crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError::DependencyAccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::dependency_access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_dependency_access_denied_exception::de_dependency_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "IllegalArgumentException" => crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError::IllegalArgumentException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::illegal_argument_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_illegal_argument_exception::de_illegal_argument_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoAvailableOrganizationException" => crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError::NoAvailableOrganizationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_available_organization_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_available_organization_exception::de_no_available_organization_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchResourceException" => crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError::NoSuchResourceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_such_resource_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_resource_exception::de_no_such_resource_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceException" => crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TemplatesNotAvailableInRegionException" => crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError::TemplatesNotAvailableInRegionException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::templates_not_available_in_region_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_templates_not_available_in_region_exception::de_templates_not_available_in_region_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRequestsException" => crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_requests_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_service_quota_increase_request_from_template_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteServiceQuotaIncreaseRequestFromTemplateOutput, crate::error::DeleteServiceQuotaIncreaseRequestFromTemplateError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_service_quota_increase_request_from_template_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

