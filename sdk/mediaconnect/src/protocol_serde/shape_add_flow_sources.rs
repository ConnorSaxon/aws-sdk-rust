// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_add_flow_sources_input(input: &crate::input::AddFlowSourcesInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_add_flow_sources_input::ser_add_flow_sources_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_add_flow_sources_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AddFlowSourcesOutput, crate::error::AddFlowSourcesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::AddFlowSourcesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::AddFlowSourcesError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::error::AddFlowSourcesError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AddFlowSourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ForbiddenException" => crate::error::AddFlowSourcesError::ForbiddenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::forbidden_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_forbidden_exception::de_forbidden_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AddFlowSourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerErrorException" => crate::error::AddFlowSourcesError::InternalServerErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error_exception::de_internal_server_error_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AddFlowSourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotFoundException" => crate::error::AddFlowSourcesError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AddFlowSourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceUnavailableException" => crate::error::AddFlowSourcesError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AddFlowSourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRequestsException" => crate::error::AddFlowSourcesError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_requests_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AddFlowSourcesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::AddFlowSourcesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_add_flow_sources_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AddFlowSourcesOutput, crate::error::AddFlowSourcesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::add_flow_sources_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_add_flow_sources::de_add_flow_sources(response.body().as_ref(), output).map_err(crate::error::AddFlowSourcesError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_add_flow_sources(value: &[u8], mut builder: crate::output::add_flow_sources_output::Builder) -> Result<crate::output::add_flow_sources_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "flowArn" => {
                        builder = builder.set_flow_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "sources" => {
                        builder = builder.set_sources(
                            crate::protocol_serde::shape___list_of_source::de___list_of_source(tokens)?
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

