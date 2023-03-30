// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disassociate_configuration_items_from_application_input(input: &crate::input::DisassociateConfigurationItemsFromApplicationInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_disassociate_configuration_items_from_application_input::ser_disassociate_configuration_items_from_application_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_disassociate_configuration_items_from_application_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DisassociateConfigurationItemsFromApplicationOutput, crate::error::DisassociateConfigurationItemsFromApplicationError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DisassociateConfigurationItemsFromApplicationError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DisassociateConfigurationItemsFromApplicationError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AuthorizationErrorException" => crate::error::DisassociateConfigurationItemsFromApplicationError::AuthorizationErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::authorization_error_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_authorization_error_exception::de_authorization_error_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DisassociateConfigurationItemsFromApplicationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "HomeRegionNotSetException" => crate::error::DisassociateConfigurationItemsFromApplicationError::HomeRegionNotSetException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::home_region_not_set_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_home_region_not_set_exception::de_home_region_not_set_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DisassociateConfigurationItemsFromApplicationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterException" => crate::error::DisassociateConfigurationItemsFromApplicationError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DisassociateConfigurationItemsFromApplicationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterValueException" => crate::error::DisassociateConfigurationItemsFromApplicationError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_value_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DisassociateConfigurationItemsFromApplicationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServerInternalErrorException" => crate::error::DisassociateConfigurationItemsFromApplicationError::ServerInternalErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::server_internal_error_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_server_internal_error_exception::de_server_internal_error_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DisassociateConfigurationItemsFromApplicationError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DisassociateConfigurationItemsFromApplicationError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_disassociate_configuration_items_from_application_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DisassociateConfigurationItemsFromApplicationOutput, crate::error::DisassociateConfigurationItemsFromApplicationError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::disassociate_configuration_items_from_application_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

