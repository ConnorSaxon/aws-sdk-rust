// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_organization_config_rules_input(input: &crate::input::DescribeOrganizationConfigRulesInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_organization_config_rules_input::ser_describe_organization_config_rules_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_organization_config_rules_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeOrganizationConfigRulesOutput, crate::error::DescribeOrganizationConfigRulesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeOrganizationConfigRulesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeOrganizationConfigRulesError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidLimitException" => crate::error::DescribeOrganizationConfigRulesError::InvalidLimitException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_limit_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeOrganizationConfigRulesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidNextTokenException" => crate::error::DescribeOrganizationConfigRulesError::InvalidNextTokenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_next_token_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeOrganizationConfigRulesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchOrganizationConfigRuleException" => crate::error::DescribeOrganizationConfigRulesError::NoSuchOrganizationConfigRuleException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_such_organization_config_rule_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_organization_config_rule_exception::de_no_such_organization_config_rule_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeOrganizationConfigRulesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "OrganizationAccessDeniedException" => crate::error::DescribeOrganizationConfigRulesError::OrganizationAccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::organization_access_denied_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_organization_access_denied_exception::de_organization_access_denied_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DescribeOrganizationConfigRulesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeOrganizationConfigRulesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_organization_config_rules_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeOrganizationConfigRulesOutput, crate::error::DescribeOrganizationConfigRulesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_organization_config_rules_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_organization_config_rules::de_describe_organization_config_rules(response.body().as_ref(), output).map_err(crate::error::DescribeOrganizationConfigRulesError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_describe_organization_config_rules(value: &[u8], mut builder: crate::output::describe_organization_config_rules_output::Builder) -> Result<crate::output::describe_organization_config_rules_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "OrganizationConfigRules" => {
                        builder = builder.set_organization_config_rules(
                            crate::protocol_serde::shape_organization_config_rules::de_organization_config_rules(tokens)?
                        );
                    }
                    "NextToken" => {
                        builder = builder.set_next_token(
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

