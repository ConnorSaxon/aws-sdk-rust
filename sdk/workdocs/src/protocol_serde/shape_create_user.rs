// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_user_headers(
                    input: &crate::input::CreateUserInput,
                    mut builder: http::request::Builder
                ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::error::BuildError> {
    if let Some(inner_1) = &input.authentication_token {
        let formatted_2 = inner_1.as_str();
                        if !formatted_2.is_empty() {
                            let header_value = formatted_2;
                            let header_value: http::HeaderValue = header_value.parse().map_err(|err| {
                                aws_smithy_http::operation::error::BuildError::invalid_field("authentication_token", format!(
                                "`{}` cannot be used as a header value: {}",
                                &"*** Sensitive Data Redacted ***",
                                err
                            ))
                            })?;
                            builder = builder.header("Authentication", header_value);
                        }
    }
    Ok(builder)
}

pub fn ser_create_user_input(input: &crate::input::CreateUserInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_user_input::ser_create_user_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_user_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateUserOutput, crate::error::CreateUserError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateUserError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateUserError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "EntityAlreadyExistsException" => crate::error::CreateUserError::EntityAlreadyExistsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::entity_already_exists_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_entity_already_exists_exception::de_entity_already_exists_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "FailedDependencyException" => crate::error::CreateUserError::FailedDependencyException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::failed_dependency_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_failed_dependency_exception::de_failed_dependency_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceUnavailableException" => crate::error::CreateUserError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnauthorizedOperationException" => crate::error::CreateUserError::UnauthorizedOperationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unauthorized_operation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unauthorized_operation_exception::de_unauthorized_operation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnauthorizedResourceAccessException" => crate::error::CreateUserError::UnauthorizedResourceAccessException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unauthorized_resource_access_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unauthorized_resource_access_exception::de_unauthorized_resource_access_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreateUserError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_user_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateUserOutput, crate::error::CreateUserError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_user_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_user::de_create_user(response.body().as_ref(), output).map_err(crate::error::CreateUserError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_create_user(value: &[u8], mut builder: crate::output::create_user_output::Builder) -> Result<crate::output::create_user_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "User" => {
                        builder = builder.set_user(
                            crate::protocol_serde::shape_user::de_user(tokens)?
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

