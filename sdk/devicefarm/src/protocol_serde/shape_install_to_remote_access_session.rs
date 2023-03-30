// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_install_to_remote_access_session_input(input: &crate::input::InstallToRemoteAccessSessionInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_install_to_remote_access_session_input::ser_install_to_remote_access_session_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_install_to_remote_access_session_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::InstallToRemoteAccessSessionOutput, crate::error::InstallToRemoteAccessSessionError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::InstallToRemoteAccessSessionError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::InstallToRemoteAccessSessionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ArgumentException" => crate::error::InstallToRemoteAccessSessionError::ArgumentException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::argument_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_argument_exception::de_argument_exception_json_err(response.body().as_ref(), output).map_err(crate::error::InstallToRemoteAccessSessionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceededException" => crate::error::InstallToRemoteAccessSessionError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::InstallToRemoteAccessSessionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotFoundException" => crate::error::InstallToRemoteAccessSessionError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::InstallToRemoteAccessSessionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceAccountException" => crate::error::InstallToRemoteAccessSessionError::ServiceAccountException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_account_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_account_exception::de_service_account_exception_json_err(response.body().as_ref(), output).map_err(crate::error::InstallToRemoteAccessSessionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::InstallToRemoteAccessSessionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_install_to_remote_access_session_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::InstallToRemoteAccessSessionOutput, crate::error::InstallToRemoteAccessSessionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::install_to_remote_access_session_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_install_to_remote_access_session::de_install_to_remote_access_session(response.body().as_ref(), output).map_err(crate::error::InstallToRemoteAccessSessionError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_install_to_remote_access_session(value: &[u8], mut builder: crate::output::install_to_remote_access_session_output::Builder) -> Result<crate::output::install_to_remote_access_session_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "appUpload" => {
                        builder = builder.set_app_upload(
                            crate::protocol_serde::shape_upload::de_upload(tokens)?
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

