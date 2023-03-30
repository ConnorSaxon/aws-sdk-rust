// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_import_task_input(input: &crate::input::StartImportTaskInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_start_import_task_input::ser_start_import_task_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_import_task_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StartImportTaskOutput, crate::error::StartImportTaskError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::StartImportTaskError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::StartImportTaskError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AuthorizationErrorException" => crate::error::StartImportTaskError::AuthorizationErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::authorization_error_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_authorization_error_exception::de_authorization_error_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartImportTaskError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "HomeRegionNotSetException" => crate::error::StartImportTaskError::HomeRegionNotSetException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::home_region_not_set_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_home_region_not_set_exception::de_home_region_not_set_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartImportTaskError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterException" => crate::error::StartImportTaskError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartImportTaskError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterValueException" => crate::error::StartImportTaskError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_value_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartImportTaskError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceInUseException" => crate::error::StartImportTaskError::ResourceInUseException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_in_use_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_in_use_exception::de_resource_in_use_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartImportTaskError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServerInternalErrorException" => crate::error::StartImportTaskError::ServerInternalErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::server_internal_error_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_server_internal_error_exception::de_server_internal_error_exception_json_err(response.body().as_ref(), output).map_err(crate::error::StartImportTaskError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::StartImportTaskError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_import_task_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::StartImportTaskOutput, crate::error::StartImportTaskError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::start_import_task_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_start_import_task::de_start_import_task(response.body().as_ref(), output).map_err(crate::error::StartImportTaskError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_start_import_task(value: &[u8], mut builder: crate::output::start_import_task_output::Builder) -> Result<crate::output::start_import_task_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "task" => {
                        builder = builder.set_task(
                            crate::protocol_serde::shape_import_task::de_import_task(tokens)?
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

