// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_configuration_set_tracking_options_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateConfigurationSetTrackingOptionsOutput, crate::error::CreateConfigurationSetTrackingOptionsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateConfigurationSetTrackingOptionsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateConfigurationSetTrackingOptionsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ConfigurationSetDoesNotExist" => crate::error::CreateConfigurationSetTrackingOptionsError::ConfigurationSetDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::configuration_set_does_not_exist_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_configuration_set_does_not_exist_exception::de_configuration_set_does_not_exist_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateConfigurationSetTrackingOptionsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidTrackingOptions" => crate::error::CreateConfigurationSetTrackingOptionsError::InvalidTrackingOptionsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_tracking_options_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_tracking_options_exception::de_invalid_tracking_options_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateConfigurationSetTrackingOptionsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TrackingOptionsAlreadyExistsException" => crate::error::CreateConfigurationSetTrackingOptionsError::TrackingOptionsAlreadyExistsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::tracking_options_already_exists_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_tracking_options_already_exists_exception::de_tracking_options_already_exists_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateConfigurationSetTrackingOptionsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreateConfigurationSetTrackingOptionsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_configuration_set_tracking_options_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateConfigurationSetTrackingOptionsOutput, crate::error::CreateConfigurationSetTrackingOptionsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_configuration_set_tracking_options_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

