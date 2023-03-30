// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_size_constraint_set_input(input: &crate::input::CreateSizeConstraintSetInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_size_constraint_set_input::ser_create_size_constraint_set_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_size_constraint_set_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateSizeConstraintSetOutput, crate::error::CreateSizeConstraintSetError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateSizeConstraintSetError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateSizeConstraintSetError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "WAFDisallowedNameException" => crate::error::CreateSizeConstraintSetError::WafDisallowedNameException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_disallowed_name_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_disallowed_name_exception::de_waf_disallowed_name_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateSizeConstraintSetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFInternalErrorException" => crate::error::CreateSizeConstraintSetError::WafInternalErrorException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_internal_error_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_internal_error_exception::de_waf_internal_error_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateSizeConstraintSetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFInvalidAccountException" => crate::error::CreateSizeConstraintSetError::WafInvalidAccountException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_invalid_account_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_invalid_account_exception::de_waf_invalid_account_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateSizeConstraintSetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFInvalidParameterException" => crate::error::CreateSizeConstraintSetError::WafInvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_invalid_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_invalid_parameter_exception::de_waf_invalid_parameter_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateSizeConstraintSetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFLimitsExceededException" => crate::error::CreateSizeConstraintSetError::WafLimitsExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_limits_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_limits_exceeded_exception::de_waf_limits_exceeded_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateSizeConstraintSetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "WAFStaleDataException" => crate::error::CreateSizeConstraintSetError::WafStaleDataException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::waf_stale_data_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_waf_stale_data_exception::de_waf_stale_data_exception_json_err(response.body().as_ref(), output).map_err(crate::error::CreateSizeConstraintSetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreateSizeConstraintSetError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_size_constraint_set_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateSizeConstraintSetOutput, crate::error::CreateSizeConstraintSetError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_size_constraint_set_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_size_constraint_set::de_create_size_constraint_set(response.body().as_ref(), output).map_err(crate::error::CreateSizeConstraintSetError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_create_size_constraint_set(value: &[u8], mut builder: crate::output::create_size_constraint_set_output::Builder) -> Result<crate::output::create_size_constraint_set_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "SizeConstraintSet" => {
                        builder = builder.set_size_constraint_set(
                            crate::protocol_serde::shape_size_constraint_set::de_size_constraint_set(tokens)?
                        );
                    }
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

