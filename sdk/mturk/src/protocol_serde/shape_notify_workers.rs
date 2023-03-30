// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_notify_workers_input(input: &crate::input::NotifyWorkersInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_notify_workers_input::ser_notify_workers_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_notify_workers_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::NotifyWorkersOutput, crate::error::NotifyWorkersError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::NotifyWorkersError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::NotifyWorkersError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "RequestError" => crate::error::NotifyWorkersError::RequestError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::request_error::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_request_error::de_request_error_json_err(response.body().as_ref(), output).map_err(crate::error::NotifyWorkersError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceFault" => crate::error::NotifyWorkersError::ServiceFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_fault::de_service_fault_json_err(response.body().as_ref(), output).map_err(crate::error::NotifyWorkersError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::NotifyWorkersError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_notify_workers_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::NotifyWorkersOutput, crate::error::NotifyWorkersError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::notify_workers_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_notify_workers::de_notify_workers(response.body().as_ref(), output).map_err(crate::error::NotifyWorkersError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_notify_workers(value: &[u8], mut builder: crate::output::notify_workers_output::Builder) -> Result<crate::output::notify_workers_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "NotifyWorkersFailureStatuses" => {
                        builder = builder.set_notify_workers_failure_statuses(
                            crate::protocol_serde::shape_notify_workers_failure_status_list::de_notify_workers_failure_status_list(tokens)?
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

