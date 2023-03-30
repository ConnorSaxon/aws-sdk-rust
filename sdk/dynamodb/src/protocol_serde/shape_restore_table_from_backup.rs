// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_restore_table_from_backup_input(input: &crate::input::RestoreTableFromBackupInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_restore_table_from_backup_input::ser_restore_table_from_backup_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_restore_table_from_backup_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::RestoreTableFromBackupOutput, crate::error::RestoreTableFromBackupError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::RestoreTableFromBackupError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::RestoreTableFromBackupError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BackupInUseException" => crate::error::RestoreTableFromBackupError::BackupInUseException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::backup_in_use_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_backup_in_use_exception::de_backup_in_use_exception_json_err(response.body().as_ref(), output).map_err(crate::error::RestoreTableFromBackupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "BackupNotFoundException" => crate::error::RestoreTableFromBackupError::BackupNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::backup_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_backup_not_found_exception::de_backup_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::RestoreTableFromBackupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerError" => crate::error::RestoreTableFromBackupError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(response.body().as_ref(), output).map_err(crate::error::RestoreTableFromBackupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidEndpointException" => crate::error::RestoreTableFromBackupError::InvalidEndpointException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_endpoint_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_endpoint_exception::de_invalid_endpoint_exception_json_err(response.body().as_ref(), output).map_err(crate::error::RestoreTableFromBackupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceededException" => crate::error::RestoreTableFromBackupError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::RestoreTableFromBackupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TableAlreadyExistsException" => crate::error::RestoreTableFromBackupError::TableAlreadyExistsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::table_already_exists_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_table_already_exists_exception::de_table_already_exists_exception_json_err(response.body().as_ref(), output).map_err(crate::error::RestoreTableFromBackupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TableInUseException" => crate::error::RestoreTableFromBackupError::TableInUseException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::table_in_use_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_table_in_use_exception::de_table_in_use_exception_json_err(response.body().as_ref(), output).map_err(crate::error::RestoreTableFromBackupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::RestoreTableFromBackupError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_restore_table_from_backup_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::RestoreTableFromBackupOutput, crate::error::RestoreTableFromBackupError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::restore_table_from_backup_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_restore_table_from_backup::de_restore_table_from_backup(response.body().as_ref(), output).map_err(crate::error::RestoreTableFromBackupError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_restore_table_from_backup(value: &[u8], mut builder: crate::output::restore_table_from_backup_output::Builder) -> Result<crate::output::restore_table_from_backup_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "TableDescription" => {
                        builder = builder.set_table_description(
                            crate::protocol_serde::shape_table_description::de_table_description(tokens)?
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

