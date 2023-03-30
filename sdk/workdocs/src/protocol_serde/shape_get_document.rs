// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_document_headers(
                    input: &crate::input::GetDocumentInput,
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

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_document_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetDocumentOutput, crate::error::GetDocumentError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetDocumentError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetDocumentError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "EntityNotExistsException" => crate::error::GetDocumentError::EntityNotExistsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::entity_not_exists_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_entity_not_exists_exception::de_entity_not_exists_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetDocumentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "FailedDependencyException" => crate::error::GetDocumentError::FailedDependencyException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::failed_dependency_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_failed_dependency_exception::de_failed_dependency_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetDocumentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidArgumentException" => crate::error::GetDocumentError::InvalidArgumentException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_argument_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_argument_exception::de_invalid_argument_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetDocumentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidPasswordException" => crate::error::GetDocumentError::InvalidPasswordException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_password_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_password_exception::de_invalid_password_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetDocumentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceUnavailableException" => crate::error::GetDocumentError::ServiceUnavailableException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_unavailable_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetDocumentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnauthorizedOperationException" => crate::error::GetDocumentError::UnauthorizedOperationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unauthorized_operation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unauthorized_operation_exception::de_unauthorized_operation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetDocumentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnauthorizedResourceAccessException" => crate::error::GetDocumentError::UnauthorizedResourceAccessException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unauthorized_resource_access_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unauthorized_resource_access_exception::de_unauthorized_resource_access_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetDocumentError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GetDocumentError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_document_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetDocumentOutput, crate::error::GetDocumentError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_document_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_document::de_get_document(response.body().as_ref(), output).map_err(crate::error::GetDocumentError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_document(value: &[u8], mut builder: crate::output::get_document_output::Builder) -> Result<crate::output::get_document_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "CustomMetadata" => {
                        builder = builder.set_custom_metadata(
                            crate::protocol_serde::shape_custom_metadata_map::de_custom_metadata_map(tokens)?
                        );
                    }
                    "Metadata" => {
                        builder = builder.set_metadata(
                            crate::protocol_serde::shape_document_metadata::de_document_metadata(tokens)?
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

