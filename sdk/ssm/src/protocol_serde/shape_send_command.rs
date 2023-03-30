// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_send_command_input(input: &crate::input::SendCommandInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_send_command_input::ser_send_command_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_send_command_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::SendCommandOutput, crate::error::SendCommandError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::SendCommandError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::SendCommandError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "DuplicateInstanceId" => crate::error::SendCommandError::DuplicateInstanceId({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::duplicate_instance_id::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_duplicate_instance_id::de_duplicate_instance_id_json_err(response.body().as_ref(), output).map_err(crate::error::SendCommandError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerError" => crate::error::SendCommandError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(response.body().as_ref(), output).map_err(crate::error::SendCommandError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidDocument" => crate::error::SendCommandError::InvalidDocument({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_document::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_document::de_invalid_document_json_err(response.body().as_ref(), output).map_err(crate::error::SendCommandError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidDocumentVersion" => crate::error::SendCommandError::InvalidDocumentVersion({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_document_version::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_document_version::de_invalid_document_version_json_err(response.body().as_ref(), output).map_err(crate::error::SendCommandError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInstanceId" => crate::error::SendCommandError::InvalidInstanceId({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_instance_id::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_instance_id::de_invalid_instance_id_json_err(response.body().as_ref(), output).map_err(crate::error::SendCommandError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidNotificationConfig" => crate::error::SendCommandError::InvalidNotificationConfig({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_notification_config::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_notification_config::de_invalid_notification_config_json_err(response.body().as_ref(), output).map_err(crate::error::SendCommandError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidOutputFolder" => crate::error::SendCommandError::InvalidOutputFolder({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_output_folder::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_output_folder::de_invalid_output_folder_json_err(response.body().as_ref(), output).map_err(crate::error::SendCommandError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameters" => crate::error::SendCommandError::InvalidParameters({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameters::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameters::de_invalid_parameters_json_err(response.body().as_ref(), output).map_err(crate::error::SendCommandError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidRole" => crate::error::SendCommandError::InvalidRole({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_role::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_role::de_invalid_role_json_err(response.body().as_ref(), output).map_err(crate::error::SendCommandError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "MaxDocumentSizeExceeded" => crate::error::SendCommandError::MaxDocumentSizeExceeded({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::max_document_size_exceeded::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_max_document_size_exceeded::de_max_document_size_exceeded_json_err(response.body().as_ref(), output).map_err(crate::error::SendCommandError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedPlatformType" => crate::error::SendCommandError::UnsupportedPlatformType({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unsupported_platform_type::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_platform_type::de_unsupported_platform_type_json_err(response.body().as_ref(), output).map_err(crate::error::SendCommandError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::SendCommandError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_send_command_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::SendCommandOutput, crate::error::SendCommandError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::send_command_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_send_command::de_send_command(response.body().as_ref(), output).map_err(crate::error::SendCommandError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_send_command(value: &[u8], mut builder: crate::output::send_command_output::Builder) -> Result<crate::output::send_command_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Command" => {
                        builder = builder.set_command(
                            crate::protocol_serde::shape_command::de_command(tokens)?
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

