// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_associate_service_quota_template_input(_input: &crate::input::AssociateServiceQuotaTemplateInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    Ok(aws_smithy_http::body::SdkBody::from("{}"))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_service_quota_template_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AssociateServiceQuotaTemplateOutput, crate::error::AssociateServiceQuotaTemplateError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::AssociateServiceQuotaTemplateError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::AssociateServiceQuotaTemplateError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::AssociateServiceQuotaTemplateError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AssociateServiceQuotaTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "AWSServiceAccessNotEnabledException" => crate::error::AssociateServiceQuotaTemplateError::AwsServiceAccessNotEnabledException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::aws_service_access_not_enabled_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_aws_service_access_not_enabled_exception::de_aws_service_access_not_enabled_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AssociateServiceQuotaTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DependencyAccessDeniedException" => crate::error::AssociateServiceQuotaTemplateError::DependencyAccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::dependency_access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_dependency_access_denied_exception::de_dependency_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AssociateServiceQuotaTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoAvailableOrganizationException" => crate::error::AssociateServiceQuotaTemplateError::NoAvailableOrganizationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_available_organization_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_available_organization_exception::de_no_available_organization_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AssociateServiceQuotaTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OrganizationNotInAllFeaturesModeException" => crate::error::AssociateServiceQuotaTemplateError::OrganizationNotInAllFeaturesModeException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::organization_not_in_all_features_mode_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_organization_not_in_all_features_mode_exception::de_organization_not_in_all_features_mode_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AssociateServiceQuotaTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceException" => crate::error::AssociateServiceQuotaTemplateError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AssociateServiceQuotaTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TemplatesNotAvailableInRegionException" => crate::error::AssociateServiceQuotaTemplateError::TemplatesNotAvailableInRegionException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::templates_not_available_in_region_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_templates_not_available_in_region_exception::de_templates_not_available_in_region_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AssociateServiceQuotaTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRequestsException" => crate::error::AssociateServiceQuotaTemplateError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_requests_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AssociateServiceQuotaTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::AssociateServiceQuotaTemplateError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_associate_service_quota_template_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AssociateServiceQuotaTemplateOutput, crate::error::AssociateServiceQuotaTemplateError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::associate_service_quota_template_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

