// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_web_acl_input(input: &crate::input::UpdateWebAclInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_web_acl_input::ser_update_web_acl_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_web_acl_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateWebAclOutput, crate::error::UpdateWebACLError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::UpdateWebACLError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UpdateWebACLError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "WAFInternalErrorException" => crate::error::UpdateWebACLError::WafInternalErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_internal_error_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_internal_error_exception::de_waf_internal_error_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateWebACLError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFInvalidAccountException" => crate::error::UpdateWebACLError::WafInvalidAccountException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_invalid_account_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_invalid_account_exception::de_waf_invalid_account_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateWebACLError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFInvalidOperationException" => crate::error::UpdateWebACLError::WafInvalidOperationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_invalid_operation_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_invalid_operation_exception::de_waf_invalid_operation_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateWebACLError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFInvalidParameterException" => crate::error::UpdateWebACLError::WafInvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_invalid_parameter_exception::de_waf_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateWebACLError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFLimitsExceededException" => crate::error::UpdateWebACLError::WafLimitsExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_limits_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_limits_exceeded_exception::de_waf_limits_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateWebACLError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFNonexistentContainerException" => crate::error::UpdateWebACLError::WafNonexistentContainerException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_nonexistent_container_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_nonexistent_container_exception::de_waf_nonexistent_container_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateWebACLError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFNonexistentItemException" => crate::error::UpdateWebACLError::WafNonexistentItemException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_nonexistent_item_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_nonexistent_item_exception::de_waf_nonexistent_item_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateWebACLError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFReferencedItemException" => crate::error::UpdateWebACLError::WafReferencedItemException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_referenced_item_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_referenced_item_exception::de_waf_referenced_item_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateWebACLError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFStaleDataException" => crate::error::UpdateWebACLError::WafStaleDataException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_stale_data_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_stale_data_exception::de_waf_stale_data_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateWebACLError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFSubscriptionNotFoundException" => crate::error::UpdateWebACLError::WafSubscriptionNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_subscription_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_subscription_not_found_exception::de_waf_subscription_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateWebACLError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::UpdateWebACLError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_web_acl_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateWebAclOutput, crate::error::UpdateWebACLError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::update_web_acl_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_update_web_acl::de_update_web_acl(response.body().as_ref(), output).map_err(crate::error::UpdateWebACLError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_update_web_acl(value: &[u8], mut builder: crate::output::update_web_acl_output::Builder) -> Result<crate::output::update_web_acl_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "ChangeToken" => {
                        builder = builder.set_change_token(
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

