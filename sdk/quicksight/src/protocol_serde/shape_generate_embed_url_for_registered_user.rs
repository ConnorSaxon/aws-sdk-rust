// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_generate_embed_url_for_registered_user_input(input: &crate::input::GenerateEmbedUrlForRegisteredUserInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_generate_embed_url_for_registered_user_input::ser_generate_embed_url_for_registered_user_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_generate_embed_url_for_registered_user_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GenerateEmbedUrlForRegisteredUserOutput, crate::error::GenerateEmbedUrlForRegisteredUserError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GenerateEmbedUrlForRegisteredUserError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GenerateEmbedUrlForRegisteredUserError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::error::GenerateEmbedUrlForRegisteredUserError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GenerateEmbedUrlForRegisteredUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InternalFailureException" => crate::error::GenerateEmbedUrlForRegisteredUserError::InternalFailureException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_failure_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_failure_exception::de_internal_failure_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GenerateEmbedUrlForRegisteredUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterValueException" => crate::error::GenerateEmbedUrlForRegisteredUserError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_value_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GenerateEmbedUrlForRegisteredUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "QuickSightUserNotFoundException" => crate::error::GenerateEmbedUrlForRegisteredUserError::QuickSightUserNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::quick_sight_user_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_quick_sight_user_not_found_exception::de_quick_sight_user_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GenerateEmbedUrlForRegisteredUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::GenerateEmbedUrlForRegisteredUserError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GenerateEmbedUrlForRegisteredUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "SessionLifetimeInMinutesInvalidException" => crate::error::GenerateEmbedUrlForRegisteredUserError::SessionLifetimeInMinutesInvalidException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::session_lifetime_in_minutes_invalid_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_session_lifetime_in_minutes_invalid_exception::de_session_lifetime_in_minutes_invalid_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GenerateEmbedUrlForRegisteredUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ThrottlingException" => crate::error::GenerateEmbedUrlForRegisteredUserError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GenerateEmbedUrlForRegisteredUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedPricingPlanException" => crate::error::GenerateEmbedUrlForRegisteredUserError::UnsupportedPricingPlanException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unsupported_pricing_plan_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_pricing_plan_exception::de_unsupported_pricing_plan_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GenerateEmbedUrlForRegisteredUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedUserEditionException" => crate::error::GenerateEmbedUrlForRegisteredUserError::UnsupportedUserEditionException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unsupported_user_edition_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_user_edition_exception::de_unsupported_user_edition_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GenerateEmbedUrlForRegisteredUserError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GenerateEmbedUrlForRegisteredUserError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_generate_embed_url_for_registered_user_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GenerateEmbedUrlForRegisteredUserOutput, crate::error::GenerateEmbedUrlForRegisteredUserError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::generate_embed_url_for_registered_user_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_generate_embed_url_for_registered_user::de_generate_embed_url_for_registered_user(response.body().as_ref(), output).map_err(crate::error::GenerateEmbedUrlForRegisteredUserError::unhandled)?;
        output = output.set_status(
            Some(response.status().as_u16() as _)
        );
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_generate_embed_url_for_registered_user(value: &[u8], mut builder: crate::output::generate_embed_url_for_registered_user_output::Builder) -> Result<crate::output::generate_embed_url_for_registered_user_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "EmbedUrl" => {
                        builder = builder.set_embed_url(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "RequestId" => {
                        builder = builder.set_request_id(
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

