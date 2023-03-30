// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_cancel_data_repository_task_input(input: &crate::input::CancelDataRepositoryTaskInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_cancel_data_repository_task_input::ser_cancel_data_repository_task_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_cancel_data_repository_task_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CancelDataRepositoryTaskOutput, crate::error::CancelDataRepositoryTaskError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CancelDataRepositoryTaskError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CancelDataRepositoryTaskError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BadRequest" => crate::error::CancelDataRepositoryTaskError::BadRequest({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request::de_bad_request_json_err(response.body().as_ref(), output).map_err(crate::error::CancelDataRepositoryTaskError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DataRepositoryTaskEnded" => crate::error::CancelDataRepositoryTaskError::DataRepositoryTaskEnded({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::data_repository_task_ended::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_data_repository_task_ended::de_data_repository_task_ended_json_err(response.body().as_ref(), output).map_err(crate::error::CancelDataRepositoryTaskError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DataRepositoryTaskNotFound" => crate::error::CancelDataRepositoryTaskError::DataRepositoryTaskNotFound({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::data_repository_task_not_found::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_data_repository_task_not_found::de_data_repository_task_not_found_json_err(response.body().as_ref(), output).map_err(crate::error::CancelDataRepositoryTaskError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerError" => crate::error::CancelDataRepositoryTaskError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(response.body().as_ref(), output).map_err(crate::error::CancelDataRepositoryTaskError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedOperation" => crate::error::CancelDataRepositoryTaskError::UnsupportedOperation({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unsupported_operation::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_operation::de_unsupported_operation_json_err(response.body().as_ref(), output).map_err(crate::error::CancelDataRepositoryTaskError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CancelDataRepositoryTaskError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_cancel_data_repository_task_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CancelDataRepositoryTaskOutput, crate::error::CancelDataRepositoryTaskError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::cancel_data_repository_task_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_cancel_data_repository_task::de_cancel_data_repository_task(response.body().as_ref(), output).map_err(crate::error::CancelDataRepositoryTaskError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_cancel_data_repository_task(value: &[u8], mut builder: crate::output::cancel_data_repository_task_output::Builder) -> Result<crate::output::cancel_data_repository_task_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Lifecycle" => {
                        builder = builder.set_lifecycle(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::DataRepositoryTaskLifecycle::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "TaskId" => {
                        builder = builder.set_task_id(
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

