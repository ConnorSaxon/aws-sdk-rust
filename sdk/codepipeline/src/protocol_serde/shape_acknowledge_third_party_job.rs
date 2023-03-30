// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_acknowledge_third_party_job_input(input: &crate::input::AcknowledgeThirdPartyJobInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_acknowledge_third_party_job_input::ser_acknowledge_third_party_job_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_acknowledge_third_party_job_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AcknowledgeThirdPartyJobOutput, crate::error::AcknowledgeThirdPartyJobError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::AcknowledgeThirdPartyJobError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::AcknowledgeThirdPartyJobError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidClientTokenException" => crate::error::AcknowledgeThirdPartyJobError::InvalidClientTokenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_client_token_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_client_token_exception::de_invalid_client_token_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AcknowledgeThirdPartyJobError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidNonceException" => crate::error::AcknowledgeThirdPartyJobError::InvalidNonceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_nonce_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_nonce_exception::de_invalid_nonce_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AcknowledgeThirdPartyJobError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "JobNotFoundException" => crate::error::AcknowledgeThirdPartyJobError::JobNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::job_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_job_not_found_exception::de_job_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AcknowledgeThirdPartyJobError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ValidationException" => crate::error::AcknowledgeThirdPartyJobError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::AcknowledgeThirdPartyJobError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::AcknowledgeThirdPartyJobError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_acknowledge_third_party_job_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::AcknowledgeThirdPartyJobOutput, crate::error::AcknowledgeThirdPartyJobError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::acknowledge_third_party_job_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_acknowledge_third_party_job::de_acknowledge_third_party_job(response.body().as_ref(), output).map_err(crate::error::AcknowledgeThirdPartyJobError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_acknowledge_third_party_job(value: &[u8], mut builder: crate::output::acknowledge_third_party_job_output::Builder) -> Result<crate::output::acknowledge_third_party_job_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "status" => {
                        builder = builder.set_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::JobStatus::from(u.as_ref())
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

