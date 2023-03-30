// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_custom_routing_endpoint_group_input(input: &crate::input::CreateCustomRoutingEndpointGroupInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_custom_routing_endpoint_group_input::ser_create_custom_routing_endpoint_group_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_custom_routing_endpoint_group_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateCustomRoutingEndpointGroupOutput, crate::error::CreateCustomRoutingEndpointGroupError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateCustomRoutingEndpointGroupError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateCustomRoutingEndpointGroupError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AcceleratorNotFoundException" => crate::error::CreateCustomRoutingEndpointGroupError::AcceleratorNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::accelerator_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_accelerator_not_found_exception::de_accelerator_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateCustomRoutingEndpointGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "AccessDeniedException" => crate::error::CreateCustomRoutingEndpointGroupError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateCustomRoutingEndpointGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EndpointGroupAlreadyExistsException" => crate::error::CreateCustomRoutingEndpointGroupError::EndpointGroupAlreadyExistsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::endpoint_group_already_exists_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_endpoint_group_already_exists_exception::de_endpoint_group_already_exists_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateCustomRoutingEndpointGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServiceErrorException" => crate::error::CreateCustomRoutingEndpointGroupError::InternalServiceErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_error_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_service_error_exception::de_internal_service_error_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateCustomRoutingEndpointGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidArgumentException" => crate::error::CreateCustomRoutingEndpointGroupError::InvalidArgumentException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_argument_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_argument_exception::de_invalid_argument_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateCustomRoutingEndpointGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidPortRangeException" => crate::error::CreateCustomRoutingEndpointGroupError::InvalidPortRangeException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_port_range_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_port_range_exception::de_invalid_port_range_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateCustomRoutingEndpointGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceededException" => crate::error::CreateCustomRoutingEndpointGroupError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateCustomRoutingEndpointGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ListenerNotFoundException" => crate::error::CreateCustomRoutingEndpointGroupError::ListenerNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::listener_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_listener_not_found_exception::de_listener_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateCustomRoutingEndpointGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreateCustomRoutingEndpointGroupError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_custom_routing_endpoint_group_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateCustomRoutingEndpointGroupOutput, crate::error::CreateCustomRoutingEndpointGroupError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_custom_routing_endpoint_group_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_custom_routing_endpoint_group::de_create_custom_routing_endpoint_group(response.body().as_ref(), output).map_err(crate::error::CreateCustomRoutingEndpointGroupError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_create_custom_routing_endpoint_group(value: &[u8], mut builder: crate::output::create_custom_routing_endpoint_group_output::Builder) -> Result<crate::output::create_custom_routing_endpoint_group_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "EndpointGroup" => {
                        builder = builder.set_endpoint_group(
                            crate::protocol_serde::shape_custom_routing_endpoint_group::de_custom_routing_endpoint_group(tokens)?
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

