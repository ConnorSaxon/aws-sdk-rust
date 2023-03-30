// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_domain_nameservers_input(input: &crate::input::UpdateDomainNameserversInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_domain_nameservers_input::ser_update_domain_nameservers_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_domain_nameservers_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateDomainNameserversOutput, crate::error::UpdateDomainNameserversError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::UpdateDomainNameserversError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UpdateDomainNameserversError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "DuplicateRequest" => crate::error::UpdateDomainNameserversError::DuplicateRequest({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::duplicate_request::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_duplicate_request::de_duplicate_request_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateDomainNameserversError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInput" => crate::error::UpdateDomainNameserversError::InvalidInput({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_input::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_input::de_invalid_input_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateDomainNameserversError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OperationLimitExceeded" => crate::error::UpdateDomainNameserversError::OperationLimitExceeded({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::operation_limit_exceeded::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_operation_limit_exceeded::de_operation_limit_exceeded_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateDomainNameserversError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TLDRulesViolation" => crate::error::UpdateDomainNameserversError::TldRulesViolation({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::tld_rules_violation::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_tld_rules_violation::de_tld_rules_violation_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateDomainNameserversError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedTLD" => crate::error::UpdateDomainNameserversError::UnsupportedTld({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unsupported_tld::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_tld::de_unsupported_tld_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateDomainNameserversError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::UpdateDomainNameserversError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_domain_nameservers_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateDomainNameserversOutput, crate::error::UpdateDomainNameserversError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::update_domain_nameservers_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_update_domain_nameservers::de_update_domain_nameservers(response.body().as_ref(), output).map_err(crate::error::UpdateDomainNameserversError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_update_domain_nameservers(value: &[u8], mut builder: crate::output::update_domain_nameservers_output::Builder) -> Result<crate::output::update_domain_nameservers_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "OperationId" => {
                        builder = builder.set_operation_id(
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

