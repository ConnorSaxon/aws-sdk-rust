// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_repository_input(input: &crate::input::CreateRepositoryInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_repository_input::ser_create_repository_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_repository_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateRepositoryOutput, crate::error::CreateRepositoryError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateRepositoryError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateRepositoryError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidParameterException" => crate::error::CreateRepositoryError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateRepositoryError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidTagParameterException" => crate::error::CreateRepositoryError::InvalidTagParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_tag_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_tag_parameter_exception::de_invalid_tag_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateRepositoryError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceededException" => crate::error::CreateRepositoryError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateRepositoryError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "RepositoryAlreadyExistsException" => crate::error::CreateRepositoryError::RepositoryAlreadyExistsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::repository_already_exists_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_repository_already_exists_exception::de_repository_already_exists_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateRepositoryError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServerException" => crate::error::CreateRepositoryError::ServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::server_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_server_exception::de_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateRepositoryError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyTagsException" => crate::error::CreateRepositoryError::TooManyTagsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_tags_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_tags_exception::de_too_many_tags_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateRepositoryError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedCommandException" => crate::error::CreateRepositoryError::UnsupportedCommandException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unsupported_command_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_command_exception::de_unsupported_command_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateRepositoryError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreateRepositoryError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_repository_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateRepositoryOutput, crate::error::CreateRepositoryError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_repository_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_repository::de_create_repository(response.body().as_ref(), output).map_err(crate::error::CreateRepositoryError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_create_repository(value: &[u8], mut builder: crate::output::create_repository_output::Builder) -> Result<crate::output::create_repository_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "repository" => {
                        builder = builder.set_repository(
                            crate::protocol_serde::shape_repository::de_repository(tokens)?
                        );
                    }
                    "catalogData" => {
                        builder = builder.set_catalog_data(
                            crate::protocol_serde::shape_repository_catalog_data::de_repository_catalog_data(tokens)?
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

