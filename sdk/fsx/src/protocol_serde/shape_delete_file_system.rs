// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_file_system_input(input: &crate::input::DeleteFileSystemInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_file_system_input::ser_delete_file_system_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_file_system_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteFileSystemOutput, crate::error::DeleteFileSystemError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeleteFileSystemError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeleteFileSystemError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BadRequest" => crate::error::DeleteFileSystemError::BadRequest({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request::de_bad_request_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteFileSystemError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "FileSystemNotFound" => crate::error::DeleteFileSystemError::FileSystemNotFound({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::file_system_not_found::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_file_system_not_found::de_file_system_not_found_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteFileSystemError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "IncompatibleParameterError" => crate::error::DeleteFileSystemError::IncompatibleParameterError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::incompatible_parameter_error::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_incompatible_parameter_error::de_incompatible_parameter_error_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteFileSystemError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerError" => crate::error::DeleteFileSystemError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteFileSystemError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceLimitExceeded" => crate::error::DeleteFileSystemError::ServiceLimitExceeded({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_limit_exceeded::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_limit_exceeded::de_service_limit_exceeded_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteFileSystemError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DeleteFileSystemError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_file_system_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteFileSystemOutput, crate::error::DeleteFileSystemError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_file_system_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_delete_file_system::de_delete_file_system(response.body().as_ref(), output).map_err(crate::error::DeleteFileSystemError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_delete_file_system(value: &[u8], mut builder: crate::output::delete_file_system_output::Builder) -> Result<crate::output::delete_file_system_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "FileSystemId" => {
                        builder = builder.set_file_system_id(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "Lifecycle" => {
                        builder = builder.set_lifecycle(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::FileSystemLifecycle::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "WindowsResponse" => {
                        builder = builder.set_windows_response(
                            crate::protocol_serde::shape_delete_file_system_windows_response::de_delete_file_system_windows_response(tokens)?
                        );
                    }
                    "LustreResponse" => {
                        builder = builder.set_lustre_response(
                            crate::protocol_serde::shape_delete_file_system_lustre_response::de_delete_file_system_lustre_response(tokens)?
                        );
                    }
                    "OpenZFSResponse" => {
                        builder = builder.set_open_zfs_response(
                            crate::protocol_serde::shape_delete_file_system_open_zfs_response::de_delete_file_system_open_zfs_response(tokens)?
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

