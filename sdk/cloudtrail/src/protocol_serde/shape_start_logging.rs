// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_logging_input(input: &crate::input::StartLoggingInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_start_logging_input::ser_start_logging_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_logging_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StartLoggingOutput, crate::error::StartLoggingError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::StartLoggingError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::StartLoggingError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "CloudTrailARNInvalidException" => crate::error::StartLoggingError::CloudTrailArnInvalidException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cloud_trail_arn_invalid_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cloud_trail_arn_invalid_exception::de_cloud_trail_arn_invalid_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartLoggingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InsufficientDependencyServiceAccessPermissionException" => crate::error::StartLoggingError::InsufficientDependencyServiceAccessPermissionException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::insufficient_dependency_service_access_permission_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_insufficient_dependency_service_access_permission_exception::de_insufficient_dependency_service_access_permission_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartLoggingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidHomeRegionException" => crate::error::StartLoggingError::InvalidHomeRegionException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_home_region_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_home_region_exception::de_invalid_home_region_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartLoggingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidTrailNameException" => crate::error::StartLoggingError::InvalidTrailNameException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_trail_name_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_trail_name_exception::de_invalid_trail_name_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartLoggingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoManagementAccountSLRExistsException" => crate::error::StartLoggingError::NoManagementAccountSlrExistsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_management_account_slr_exists_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_management_account_slr_exists_exception::de_no_management_account_slr_exists_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartLoggingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotOrganizationMasterAccountException" => crate::error::StartLoggingError::NotOrganizationMasterAccountException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::not_organization_master_account_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_organization_master_account_exception::de_not_organization_master_account_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartLoggingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OperationNotPermittedException" => crate::error::StartLoggingError::OperationNotPermittedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::operation_not_permitted_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_not_permitted_exception::de_operation_not_permitted_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartLoggingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TrailNotFoundException" => crate::error::StartLoggingError::TrailNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::trail_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_trail_not_found_exception::de_trail_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartLoggingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedOperationException" => crate::error::StartLoggingError::UnsupportedOperationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unsupported_operation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartLoggingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::StartLoggingError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_logging_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StartLoggingOutput, crate::error::StartLoggingError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::start_logging_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

