// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_batch_get_triggers_input(input: &crate::input::BatchGetTriggersInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_batch_get_triggers_input::ser_batch_get_triggers_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_batch_get_triggers_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::BatchGetTriggersOutput, crate::error::BatchGetTriggersError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::BatchGetTriggersError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::BatchGetTriggersError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InternalServiceException" => crate::error::BatchGetTriggersError::InternalServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_service_exception::de_internal_service_exception_json_err(response.body().as_ref(), output).map_err(crate::error::BatchGetTriggersError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInputException" => crate::error::BatchGetTriggersError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_input_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_json_err(response.body().as_ref(), output).map_err(crate::error::BatchGetTriggersError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OperationTimeoutException" => crate::error::BatchGetTriggersError::OperationTimeoutException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::operation_timeout_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_timeout_exception::de_operation_timeout_exception_json_err(response.body().as_ref(), output).map_err(crate::error::BatchGetTriggersError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::BatchGetTriggersError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_batch_get_triggers_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::BatchGetTriggersOutput, crate::error::BatchGetTriggersError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::batch_get_triggers_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_batch_get_triggers::de_batch_get_triggers(response.body().as_ref(), output).map_err(crate::error::BatchGetTriggersError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_batch_get_triggers(value: &[u8], mut builder: crate::output::batch_get_triggers_output::Builder) -> Result<crate::output::batch_get_triggers_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Triggers" => {
                        builder = builder.set_triggers(
                            crate::protocol_serde::shape_trigger_list::de_trigger_list(tokens)?
                        );
                    }
                    "TriggersNotFound" => {
                        builder = builder.set_triggers_not_found(
                            crate::protocol_serde::shape_trigger_name_list::de_trigger_name_list(tokens)?
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

