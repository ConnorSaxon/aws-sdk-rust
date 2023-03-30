// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_unshare_directory_input(input: &crate::input::UnshareDirectoryInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_unshare_directory_input::ser_unshare_directory_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_unshare_directory_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UnshareDirectoryOutput, crate::error::UnshareDirectoryError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::UnshareDirectoryError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UnshareDirectoryError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ClientException" => crate::error::UnshareDirectoryError::ClientException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::client_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_client_exception::de_client_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UnshareDirectoryError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DirectoryNotSharedException" => crate::error::UnshareDirectoryError::DirectoryNotSharedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::directory_not_shared_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_directory_not_shared_exception::de_directory_not_shared_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UnshareDirectoryError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EntityDoesNotExistException" => crate::error::UnshareDirectoryError::EntityDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::entity_does_not_exist_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_entity_does_not_exist_exception::de_entity_does_not_exist_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UnshareDirectoryError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidTargetException" => crate::error::UnshareDirectoryError::InvalidTargetException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_target_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_target_exception::de_invalid_target_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UnshareDirectoryError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceException" => crate::error::UnshareDirectoryError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UnshareDirectoryError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::UnshareDirectoryError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_unshare_directory_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UnshareDirectoryOutput, crate::error::UnshareDirectoryError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::unshare_directory_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_unshare_directory::de_unshare_directory(response.body().as_ref(), output).map_err(crate::error::UnshareDirectoryError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_unshare_directory(value: &[u8], mut builder: crate::output::unshare_directory_output::Builder) -> Result<crate::output::unshare_directory_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "SharedDirectoryId" => {
                        builder = builder.set_shared_directory_id(
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

