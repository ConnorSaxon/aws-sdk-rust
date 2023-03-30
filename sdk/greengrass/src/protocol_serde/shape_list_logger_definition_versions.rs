// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_logger_definition_versions_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListLoggerDefinitionVersionsOutput, crate::error::ListLoggerDefinitionVersionsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ListLoggerDefinitionVersionsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ListLoggerDefinitionVersionsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::error::ListLoggerDefinitionVersionsError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ListLoggerDefinitionVersionsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ListLoggerDefinitionVersionsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_logger_definition_versions_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ListLoggerDefinitionVersionsOutput, crate::error::ListLoggerDefinitionVersionsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::list_logger_definition_versions_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_list_logger_definition_versions::de_list_logger_definition_versions(response.body().as_ref(), output).map_err(crate::error::ListLoggerDefinitionVersionsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_list_logger_definition_versions(value: &[u8], mut builder: crate::output::list_logger_definition_versions_output::Builder) -> Result<crate::output::list_logger_definition_versions_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "NextToken" => {
                        builder = builder.set_next_token(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "Versions" => {
                        builder = builder.set_versions(
                            crate::protocol_serde::shape___list_of_version_information::de___list_of_version_information(tokens)?
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

