// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_third_party_job_details_input(input: &crate::input::GetThirdPartyJobDetailsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_third_party_job_details_input::ser_get_third_party_job_details_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_third_party_job_details_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetThirdPartyJobDetailsOutput, crate::error::GetThirdPartyJobDetailsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetThirdPartyJobDetailsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetThirdPartyJobDetailsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidClientTokenException" => crate::error::GetThirdPartyJobDetailsError::InvalidClientTokenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_client_token_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_client_token_exception::de_invalid_client_token_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetThirdPartyJobDetailsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidJobException" => crate::error::GetThirdPartyJobDetailsError::InvalidJobException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_job_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_job_exception::de_invalid_job_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetThirdPartyJobDetailsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "JobNotFoundException" => crate::error::GetThirdPartyJobDetailsError::JobNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::job_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_job_not_found_exception::de_job_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetThirdPartyJobDetailsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ValidationException" => crate::error::GetThirdPartyJobDetailsError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::validation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetThirdPartyJobDetailsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GetThirdPartyJobDetailsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_third_party_job_details_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetThirdPartyJobDetailsOutput, crate::error::GetThirdPartyJobDetailsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_third_party_job_details_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_third_party_job_details::de_get_third_party_job_details(response.body().as_ref(), output).map_err(crate::error::GetThirdPartyJobDetailsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_third_party_job_details(value: &[u8], mut builder: crate::output::get_third_party_job_details_output::Builder) -> Result<crate::output::get_third_party_job_details_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "jobDetails" => {
                        builder = builder.set_job_details(
                            crate::protocol_serde::shape_third_party_job_details::de_third_party_job_details(tokens)?
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

