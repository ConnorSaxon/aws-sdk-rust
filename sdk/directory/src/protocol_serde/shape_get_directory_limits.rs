// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_directory_limits_input(_input: &crate::input::GetDirectoryLimitsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    Ok(aws_smithy_http::body::SdkBody::from("{}"))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_directory_limits_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetDirectoryLimitsOutput, crate::error::GetDirectoryLimitsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetDirectoryLimitsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetDirectoryLimitsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ClientException" => crate::error::GetDirectoryLimitsError::ClientException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::client_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_client_exception::de_client_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetDirectoryLimitsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "EntityDoesNotExistException" => crate::error::GetDirectoryLimitsError::EntityDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::entity_does_not_exist_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_entity_does_not_exist_exception::de_entity_does_not_exist_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetDirectoryLimitsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceException" => crate::error::GetDirectoryLimitsError::ServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_exception::de_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetDirectoryLimitsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GetDirectoryLimitsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_directory_limits_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetDirectoryLimitsOutput, crate::error::GetDirectoryLimitsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_directory_limits_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_directory_limits::de_get_directory_limits(response.body().as_ref(), output).map_err(crate::error::GetDirectoryLimitsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_directory_limits(value: &[u8], mut builder: crate::output::get_directory_limits_output::Builder) -> Result<crate::output::get_directory_limits_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "DirectoryLimits" => {
                        builder = builder.set_directory_limits(
                            crate::protocol_serde::shape_directory_limits::de_directory_limits(tokens)?
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

