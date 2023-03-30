// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_managed_rule_set_input(input: &crate::input::GetManagedRuleSetInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_managed_rule_set_input::ser_get_managed_rule_set_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_managed_rule_set_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetManagedRuleSetOutput, crate::error::GetManagedRuleSetError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetManagedRuleSetError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetManagedRuleSetError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "WAFInternalErrorException" => crate::error::GetManagedRuleSetError::WafInternalErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_internal_error_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_internal_error_exception::de_waf_internal_error_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetManagedRuleSetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFInvalidOperationException" => crate::error::GetManagedRuleSetError::WafInvalidOperationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_invalid_operation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_invalid_operation_exception::de_waf_invalid_operation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetManagedRuleSetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFInvalidParameterException" => crate::error::GetManagedRuleSetError::WafInvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_invalid_parameter_exception::de_waf_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetManagedRuleSetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFNonexistentItemException" => crate::error::GetManagedRuleSetError::WafNonexistentItemException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_nonexistent_item_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_nonexistent_item_exception::de_waf_nonexistent_item_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetManagedRuleSetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GetManagedRuleSetError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_managed_rule_set_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetManagedRuleSetOutput, crate::error::GetManagedRuleSetError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_managed_rule_set_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_managed_rule_set::de_get_managed_rule_set(response.body().as_ref(), output).map_err(crate::error::GetManagedRuleSetError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_managed_rule_set(value: &[u8], mut builder: crate::output::get_managed_rule_set_output::Builder) -> Result<crate::output::get_managed_rule_set_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "ManagedRuleSet" => {
                        builder = builder.set_managed_rule_set(
                            crate::protocol_serde::shape_managed_rule_set::de_managed_rule_set(tokens)?
                        );
                    }
                    "LockToken" => {
                        builder = builder.set_lock_token(
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

