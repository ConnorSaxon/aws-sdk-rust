// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_firewall_policy_change_protection_input(input: &crate::input::UpdateFirewallPolicyChangeProtectionInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_firewall_policy_change_protection_input::ser_update_firewall_policy_change_protection_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_firewall_policy_change_protection_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateFirewallPolicyChangeProtectionOutput, crate::error::UpdateFirewallPolicyChangeProtectionError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::UpdateFirewallPolicyChangeProtectionError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::UpdateFirewallPolicyChangeProtectionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InternalServerError" => crate::error::UpdateFirewallPolicyChangeProtectionError::InternalServerError({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::internal_server_error::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_internal_server_error::de_internal_server_error_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateFirewallPolicyChangeProtectionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidRequestException" => crate::error::UpdateFirewallPolicyChangeProtectionError::InvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateFirewallPolicyChangeProtectionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidTokenException" => crate::error::UpdateFirewallPolicyChangeProtectionError::InvalidTokenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_token_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_token_exception::de_invalid_token_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateFirewallPolicyChangeProtectionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceNotFoundException" => crate::error::UpdateFirewallPolicyChangeProtectionError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateFirewallPolicyChangeProtectionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ResourceOwnerCheckException" => crate::error::UpdateFirewallPolicyChangeProtectionError::ResourceOwnerCheckException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_owner_check_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_resource_owner_check_exception::de_resource_owner_check_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateFirewallPolicyChangeProtectionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ThrottlingException" => crate::error::UpdateFirewallPolicyChangeProtectionError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::throttling_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(response.body().as_ref(), output).map_err(crate::error::UpdateFirewallPolicyChangeProtectionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::UpdateFirewallPolicyChangeProtectionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_firewall_policy_change_protection_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::UpdateFirewallPolicyChangeProtectionOutput, crate::error::UpdateFirewallPolicyChangeProtectionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::update_firewall_policy_change_protection_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_update_firewall_policy_change_protection::de_update_firewall_policy_change_protection(response.body().as_ref(), output).map_err(crate::error::UpdateFirewallPolicyChangeProtectionError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_update_firewall_policy_change_protection(value: &[u8], mut builder: crate::output::update_firewall_policy_change_protection_output::Builder) -> Result<crate::output::update_firewall_policy_change_protection_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "UpdateToken" => {
                        builder = builder.set_update_token(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "FirewallArn" => {
                        builder = builder.set_firewall_arn(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "FirewallName" => {
                        builder = builder.set_firewall_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "FirewallPolicyChangeProtection" => {
                        builder = builder.set_firewall_policy_change_protection(
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

