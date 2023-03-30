// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_cancel_journal_kinesis_stream_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CancelJournalKinesisStreamOutput, crate::error::CancelJournalKinesisStreamError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CancelJournalKinesisStreamError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CancelJournalKinesisStreamError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidParameterException" => crate::error::CancelJournalKinesisStreamError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CancelJournalKinesisStreamError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::CancelJournalKinesisStreamError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CancelJournalKinesisStreamError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourcePreconditionNotMetException" => crate::error::CancelJournalKinesisStreamError::ResourcePreconditionNotMetException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_precondition_not_met_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_precondition_not_met_exception::de_resource_precondition_not_met_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CancelJournalKinesisStreamError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CancelJournalKinesisStreamError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_cancel_journal_kinesis_stream_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CancelJournalKinesisStreamOutput, crate::error::CancelJournalKinesisStreamError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::cancel_journal_kinesis_stream_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_cancel_journal_kinesis_stream::de_cancel_journal_kinesis_stream(response.body().as_ref(), output).map_err(crate::error::CancelJournalKinesisStreamError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_cancel_journal_kinesis_stream(value: &[u8], mut builder: crate::output::cancel_journal_kinesis_stream_output::Builder) -> Result<crate::output::cancel_journal_kinesis_stream_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "StreamId" => {
                        builder = builder.set_stream_id(
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

