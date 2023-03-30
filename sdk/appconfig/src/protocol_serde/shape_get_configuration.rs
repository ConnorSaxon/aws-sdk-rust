// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_configuration_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetConfigurationOutput, crate::error::GetConfigurationError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetConfigurationError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetConfigurationError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::error::GetConfigurationError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerException" => crate::error::GetConfigurationError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::GetConfigurationError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetConfigurationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GetConfigurationError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_configuration_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetConfigurationOutput, crate::error::GetConfigurationError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_configuration_output::Builder::default();
        let _ = response;
        output = output.set_configuration_version(
            crate::protocol_serde::shape_get_configuration_output::de_configuration_version_header(response.headers())
                                    .map_err(|_|crate::error::GetConfigurationError::unhandled("Failed to parse ConfigurationVersion from header `Configuration-Version"))?
        );
        output = output.set_content(
            crate::protocol_serde::shape_get_configuration_output::de_content_payload(response.body().as_ref())?
        );
        output = output.set_content_type(
            crate::protocol_serde::shape_get_configuration_output::de_content_type_header(response.headers())
                                    .map_err(|_|crate::error::GetConfigurationError::unhandled("Failed to parse ContentType from header `Content-Type"))?
        );
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

