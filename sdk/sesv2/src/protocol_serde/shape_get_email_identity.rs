// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_email_identity_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetEmailIdentityOutput, crate::error::GetEmailIdentityError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetEmailIdentityError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetEmailIdentityError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BadRequestException" => crate::error::GetEmailIdentityError::BadRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::bad_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_bad_request_exception::de_bad_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetEmailIdentityError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NotFoundException" => crate::error::GetEmailIdentityError::NotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_not_found_exception::de_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetEmailIdentityError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TooManyRequestsException" => crate::error::GetEmailIdentityError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::too_many_requests_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetEmailIdentityError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GetEmailIdentityError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_email_identity_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetEmailIdentityOutput, crate::error::GetEmailIdentityError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_email_identity_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_email_identity::de_get_email_identity(response.body().as_ref(), output).map_err(crate::error::GetEmailIdentityError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_email_identity(value: &[u8], mut builder: crate::output::get_email_identity_output::Builder) -> Result<crate::output::get_email_identity_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "ConfigurationSetName" => {
                        builder = builder.set_configuration_set_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "DkimAttributes" => {
                        builder = builder.set_dkim_attributes(
                            crate::protocol_serde::shape_dkim_attributes::de_dkim_attributes(tokens)?
                        );
                    }
                    "FeedbackForwardingStatus" => {
                        builder = builder.set_feedback_forwarding_status(
                            aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                        );
                    }
                    "IdentityType" => {
                        builder = builder.set_identity_type(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::IdentityType::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "MailFromAttributes" => {
                        builder = builder.set_mail_from_attributes(
                            crate::protocol_serde::shape_mail_from_attributes::de_mail_from_attributes(tokens)?
                        );
                    }
                    "Policies" => {
                        builder = builder.set_policies(
                            crate::protocol_serde::shape_policy_map::de_policy_map(tokens)?
                        );
                    }
                    "Tags" => {
                        builder = builder.set_tags(
                            crate::protocol_serde::shape_tag_list::de_tag_list(tokens)?
                        );
                    }
                    "VerificationStatus" => {
                        builder = builder.set_verification_status(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    crate::model::VerificationStatus::from(u.as_ref())
                                )
                            ).transpose()?
                        );
                    }
                    "VerifiedForSendingStatus" => {
                        builder = builder.set_verified_for_sending_status(
                            aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
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

