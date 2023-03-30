// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_batch_update_findings_input(input: &crate::input::BatchUpdateFindingsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_batch_update_findings_input::ser_batch_update_findings_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_batch_update_findings_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::BatchUpdateFindingsOutput, crate::error::BatchUpdateFindingsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::BatchUpdateFindingsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::BatchUpdateFindingsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InternalException" => crate::error::BatchUpdateFindingsError::InternalException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_exception::de_internal_exception_json_err(response.body().as_ref(), output).map_err(crate::error::BatchUpdateFindingsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidAccessException" => crate::error::BatchUpdateFindingsError::InvalidAccessException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_access_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_access_exception::de_invalid_access_exception_json_err(response.body().as_ref(), output).map_err(crate::error::BatchUpdateFindingsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInputException" => crate::error::BatchUpdateFindingsError::InvalidInputException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_input_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input_exception::de_invalid_input_exception_json_err(response.body().as_ref(), output).map_err(crate::error::BatchUpdateFindingsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceededException" => crate::error::BatchUpdateFindingsError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::BatchUpdateFindingsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::BatchUpdateFindingsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_batch_update_findings_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::BatchUpdateFindingsOutput, crate::error::BatchUpdateFindingsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::batch_update_findings_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_batch_update_findings::de_batch_update_findings(response.body().as_ref(), output).map_err(crate::error::BatchUpdateFindingsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_batch_update_findings(value: &[u8], mut builder: crate::output::batch_update_findings_output::Builder) -> Result<crate::output::batch_update_findings_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "ProcessedFindings" => {
                        builder = builder.set_processed_findings(
                            crate::protocol_serde::shape_aws_security_finding_identifier_list::de_aws_security_finding_identifier_list(tokens)?
                        );
                    }
                    "UnprocessedFindings" => {
                        builder = builder.set_unprocessed_findings(
                            crate::protocol_serde::shape_batch_update_findings_unprocessed_findings_list::de_batch_update_findings_unprocessed_findings_list(tokens)?
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

