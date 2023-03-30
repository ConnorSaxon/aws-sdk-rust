// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_service_quota_increase_request_into_template_input(input: &crate::input::PutServiceQuotaIncreaseRequestIntoTemplateInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_put_service_quota_increase_request_into_template_input::ser_put_service_quota_increase_request_into_template_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_service_quota_increase_request_into_template_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PutServiceQuotaIncreaseRequestIntoTemplateOutput, crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "AWSServiceAccessNotEnabledException" => crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::AwsServiceAccessNotEnabledException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::aws_service_access_not_enabled_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_aws_service_access_not_enabled_exception::de_aws_service_access_not_enabled_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DependencyAccessDeniedException" => crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::DependencyAccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::dependency_access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_dependency_access_denied_exception::de_dependency_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "IllegalArgumentException" => crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::IllegalArgumentException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::illegal_argument_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_illegal_argument_exception::de_illegal_argument_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoAvailableOrganizationException" => crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::NoAvailableOrganizationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_available_organization_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_available_organization_exception::de_no_available_organization_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchResourceException" => crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::NoSuchResourceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_such_resource_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_resource_exception::de_no_such_resource_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "QuotaExceededException" => crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::QuotaExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::quota_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_quota_exceeded_exception::de_quota_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceException" => crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TemplatesNotAvailableInRegionException" => crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::TemplatesNotAvailableInRegionException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::templates_not_available_in_region_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_templates_not_available_in_region_exception::de_templates_not_available_in_region_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRequestsException" => crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_requests_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_service_quota_increase_request_into_template_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PutServiceQuotaIncreaseRequestIntoTemplateOutput, crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::put_service_quota_increase_request_into_template_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_put_service_quota_increase_request_into_template::de_put_service_quota_increase_request_into_template(response.body().as_ref(), output).map_err(crate::error::PutServiceQuotaIncreaseRequestIntoTemplateError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_put_service_quota_increase_request_into_template(value: &[u8], mut builder: crate::output::put_service_quota_increase_request_into_template_output::Builder) -> Result<crate::output::put_service_quota_increase_request_into_template_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "ServiceQuotaIncreaseRequestInTemplate" => {
                        builder = builder.set_service_quota_increase_request_in_template(
                            crate::protocol_serde::shape_service_quota_increase_request_in_template::de_service_quota_increase_request_in_template(tokens)?
                        );
                    }
                    _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                }
            }
            other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
        }
    }
    if tokens.next().is_some() {
        return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("found more JSON tokens after completing parsing"));
    }
    Ok(builder)
}

