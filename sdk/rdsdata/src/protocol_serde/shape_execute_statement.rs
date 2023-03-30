// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_execute_statement_input(input: &crate::input::ExecuteStatementInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_execute_statement_input::ser_execute_statement_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_execute_statement_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ExecuteStatementOutput, crate::error::ExecuteStatementError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ExecuteStatementError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ExecuteStatementError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::ExecuteStatementError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ExecuteStatementError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "BadRequestException" => crate::error::ExecuteStatementError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ExecuteStatementError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ForbiddenException" => crate::error::ExecuteStatementError::ForbiddenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::forbidden_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_forbidden_exception::de_forbidden_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ExecuteStatementError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalServerErrorException" => crate::error::ExecuteStatementError::InternalServerErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error_exception::de_internal_server_error_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ExecuteStatementError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ServiceUnavailableError" => crate::error::ExecuteStatementError::ServiceUnavailableError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::service_unavailable_error::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_service_unavailable_error::de_service_unavailable_error_json_err(response.body().as_ref(), output).map_err(crate::error::ExecuteStatementError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "StatementTimeoutException" => crate::error::ExecuteStatementError::StatementTimeoutException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::statement_timeout_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_statement_timeout_exception::de_statement_timeout_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ExecuteStatementError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ExecuteStatementError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_execute_statement_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ExecuteStatementOutput, crate::error::ExecuteStatementError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::execute_statement_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_execute_statement::de_execute_statement(response.body().as_ref(), output).map_err(crate::error::ExecuteStatementError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_execute_statement(value: &[u8], mut builder: crate::output::execute_statement_output::Builder) -> Result<crate::output::execute_statement_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "columnMetadata" => {
                        builder = builder.set_column_metadata(
                            crate::protocol_serde::shape_metadata::de_metadata(tokens)?
                        );
                    }
                    "formattedRecords" => {
                        builder = builder.set_formatted_records(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "generatedFields" => {
                        builder = builder.set_generated_fields(
                            crate::protocol_serde::shape_field_list::de_field_list(tokens)?
                        );
                    }
                    "numberOfRecordsUpdated" => {
                        builder = builder.set_number_of_records_updated(
                            aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                .map(i64::try_from)
                                                .transpose()?
                        );
                    }
                    "records" => {
                        builder = builder.set_records(
                            crate::protocol_serde::shape_sql_records::de_sql_records(tokens)?
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

