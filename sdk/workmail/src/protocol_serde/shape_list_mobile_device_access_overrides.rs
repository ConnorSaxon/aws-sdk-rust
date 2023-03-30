// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_mobile_device_access_overrides_input(input: &crate::input::ListMobileDeviceAccessOverridesInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_list_mobile_device_access_overrides_input::ser_list_mobile_device_access_overrides_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_mobile_device_access_overrides_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListMobileDeviceAccessOverridesOutput, crate::error::ListMobileDeviceAccessOverridesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ListMobileDeviceAccessOverridesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ListMobileDeviceAccessOverridesError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "EntityNotFoundException" => crate::error::ListMobileDeviceAccessOverridesError::EntityNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::entity_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_entity_not_found_exception::de_entity_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListMobileDeviceAccessOverridesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterException" => crate::error::ListMobileDeviceAccessOverridesError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListMobileDeviceAccessOverridesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OrganizationNotFoundException" => crate::error::ListMobileDeviceAccessOverridesError::OrganizationNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::organization_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_organization_not_found_exception::de_organization_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListMobileDeviceAccessOverridesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OrganizationStateException" => crate::error::ListMobileDeviceAccessOverridesError::OrganizationStateException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::organization_state_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_organization_state_exception::de_organization_state_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListMobileDeviceAccessOverridesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ListMobileDeviceAccessOverridesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_mobile_device_access_overrides_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListMobileDeviceAccessOverridesOutput, crate::error::ListMobileDeviceAccessOverridesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_mobile_device_access_overrides_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_list_mobile_device_access_overrides::de_list_mobile_device_access_overrides(response.body().as_ref(), output).map_err(crate::error::ListMobileDeviceAccessOverridesError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_list_mobile_device_access_overrides(value: &[u8], mut builder: crate::output::list_mobile_device_access_overrides_output::Builder) -> Result<crate::output::list_mobile_device_access_overrides_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Overrides" => {
                        builder = builder.set_overrides(
                            crate::protocol_serde::shape_mobile_device_access_overrides_list::de_mobile_device_access_overrides_list(tokens)?
                        );
                    }
                    "NextToken" => {
                        builder = builder.set_next_token(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
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

