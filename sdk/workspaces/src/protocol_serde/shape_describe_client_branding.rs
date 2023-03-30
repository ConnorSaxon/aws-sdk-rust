// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_client_branding_input(input: &crate::input::DescribeClientBrandingInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_client_branding_input::ser_describe_client_branding_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_client_branding_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeClientBrandingOutput, crate::error::DescribeClientBrandingError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeClientBrandingError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeClientBrandingError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::DescribeClientBrandingError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeClientBrandingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterValuesException" => crate::error::DescribeClientBrandingError::InvalidParameterValuesException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_values_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_values_exception::de_invalid_parameter_values_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeClientBrandingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::DescribeClientBrandingError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeClientBrandingError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeClientBrandingError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_client_branding_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeClientBrandingOutput, crate::error::DescribeClientBrandingError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_client_branding_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_client_branding::de_describe_client_branding(response.body().as_ref(), output).map_err(crate::error::DescribeClientBrandingError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_describe_client_branding(value: &[u8], mut builder: crate::output::describe_client_branding_output::Builder) -> Result<crate::output::describe_client_branding_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "DeviceTypeWindows" => {
                        builder = builder.set_device_type_windows(
                            crate::protocol_serde::shape_default_client_branding_attributes::de_default_client_branding_attributes(tokens)?
                        );
                    }
                    "DeviceTypeOsx" => {
                        builder = builder.set_device_type_osx(
                            crate::protocol_serde::shape_default_client_branding_attributes::de_default_client_branding_attributes(tokens)?
                        );
                    }
                    "DeviceTypeAndroid" => {
                        builder = builder.set_device_type_android(
                            crate::protocol_serde::shape_default_client_branding_attributes::de_default_client_branding_attributes(tokens)?
                        );
                    }
                    "DeviceTypeIos" => {
                        builder = builder.set_device_type_ios(
                            crate::protocol_serde::shape_ios_client_branding_attributes::de_ios_client_branding_attributes(tokens)?
                        );
                    }
                    "DeviceTypeLinux" => {
                        builder = builder.set_device_type_linux(
                            crate::protocol_serde::shape_default_client_branding_attributes::de_default_client_branding_attributes(tokens)?
                        );
                    }
                    "DeviceTypeWeb" => {
                        builder = builder.set_device_type_web(
                            crate::protocol_serde::shape_default_client_branding_attributes::de_default_client_branding_attributes(tokens)?
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

