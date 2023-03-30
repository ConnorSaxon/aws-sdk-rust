// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_contains_pii_entities_input(input: &crate::input::ContainsPiiEntitiesInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_contains_pii_entities_input::ser_contains_pii_entities_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_contains_pii_entities_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ContainsPiiEntitiesOutput, crate::error::ContainsPiiEntitiesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ContainsPiiEntitiesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ContainsPiiEntitiesError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InternalServerException" => crate::error::ContainsPiiEntitiesError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ContainsPiiEntitiesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidRequestException" => crate::error::ContainsPiiEntitiesError::InvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ContainsPiiEntitiesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TextSizeLimitExceededException" => crate::error::ContainsPiiEntitiesError::TextSizeLimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::text_size_limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_text_size_limit_exceeded_exception::de_text_size_limit_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ContainsPiiEntitiesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedLanguageException" => crate::error::ContainsPiiEntitiesError::UnsupportedLanguageException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unsupported_language_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_language_exception::de_unsupported_language_exception_json_err(response.body().as_ref(), output).map_err(crate::error::ContainsPiiEntitiesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ContainsPiiEntitiesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_contains_pii_entities_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ContainsPiiEntitiesOutput, crate::error::ContainsPiiEntitiesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::contains_pii_entities_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_contains_pii_entities::de_contains_pii_entities(response.body().as_ref(), output).map_err(crate::error::ContainsPiiEntitiesError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_contains_pii_entities(value: &[u8], mut builder: crate::output::contains_pii_entities_output::Builder) -> Result<crate::output::contains_pii_entities_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "Labels" => {
                        builder = builder.set_labels(
                            crate::protocol_serde::shape_list_of_entity_labels::de_list_of_entity_labels(tokens)?
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

