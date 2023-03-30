// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_count_pending_decision_tasks_input(input: &crate::input::CountPendingDecisionTasksInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_count_pending_decision_tasks_input::ser_count_pending_decision_tasks_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_count_pending_decision_tasks_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CountPendingDecisionTasksOutput, crate::error::CountPendingDecisionTasksError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CountPendingDecisionTasksError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CountPendingDecisionTasksError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "OperationNotPermittedFault" => crate::error::CountPendingDecisionTasksError::OperationNotPermittedFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::operation_not_permitted_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_not_permitted_fault::de_operation_not_permitted_fault_json_err(response.body().as_ref(), output).map_err(crate::error::CountPendingDecisionTasksError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnknownResourceFault" => crate::error::CountPendingDecisionTasksError::UnknownResourceFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unknown_resource_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unknown_resource_fault::de_unknown_resource_fault_json_err(response.body().as_ref(), output).map_err(crate::error::CountPendingDecisionTasksError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CountPendingDecisionTasksError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_count_pending_decision_tasks_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CountPendingDecisionTasksOutput, crate::error::CountPendingDecisionTasksError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::count_pending_decision_tasks_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_count_pending_decision_tasks::de_count_pending_decision_tasks(response.body().as_ref(), output).map_err(crate::error::CountPendingDecisionTasksError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_count_pending_decision_tasks(value: &[u8], mut builder: crate::output::count_pending_decision_tasks_output::Builder) -> Result<crate::output::count_pending_decision_tasks_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "count" => {
                        builder = builder.set_count(
                            aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                .map(i32::try_from)
                                                .transpose()?
                        );
                    }
                    "truncated" => {
                        builder = builder.set_truncated(
                            aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
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

