// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_conformance_pack_compliance_details_input(input: &crate::input::GetConformancePackComplianceDetailsInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_conformance_pack_compliance_details_input::ser_get_conformance_pack_compliance_details_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_conformance_pack_compliance_details_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetConformancePackComplianceDetailsOutput, crate::error::GetConformancePackComplianceDetailsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::GetConformancePackComplianceDetailsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::GetConformancePackComplianceDetailsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidLimitException" => crate::error::GetConformancePackComplianceDetailsError::InvalidLimitException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_limit_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_limit_exception::de_invalid_limit_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetConformancePackComplianceDetailsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidNextTokenException" => crate::error::GetConformancePackComplianceDetailsError::InvalidNextTokenException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_next_token_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_next_token_exception::de_invalid_next_token_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetConformancePackComplianceDetailsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterValueException" => crate::error::GetConformancePackComplianceDetailsError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_value_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetConformancePackComplianceDetailsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchConfigRuleInConformancePackException" => crate::error::GetConformancePackComplianceDetailsError::NoSuchConfigRuleInConformancePackException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_such_config_rule_in_conformance_pack_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_config_rule_in_conformance_pack_exception::de_no_such_config_rule_in_conformance_pack_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetConformancePackComplianceDetailsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NoSuchConformancePackException" => crate::error::GetConformancePackComplianceDetailsError::NoSuchConformancePackException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::no_such_conformance_pack_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_no_such_conformance_pack_exception::de_no_such_conformance_pack_exception_json_err(response.body().as_ref(), output).map_err(crate::error::GetConformancePackComplianceDetailsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::GetConformancePackComplianceDetailsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_conformance_pack_compliance_details_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::GetConformancePackComplianceDetailsOutput, crate::error::GetConformancePackComplianceDetailsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_conformance_pack_compliance_details_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_get_conformance_pack_compliance_details::de_get_conformance_pack_compliance_details(response.body().as_ref(), output).map_err(crate::error::GetConformancePackComplianceDetailsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_get_conformance_pack_compliance_details(value: &[u8], mut builder: crate::output::get_conformance_pack_compliance_details_output::Builder) -> Result<crate::output::get_conformance_pack_compliance_details_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "ConformancePackName" => {
                        builder = builder.set_conformance_pack_name(
                            aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                s.to_unescaped().map(|u|
                                    u.into_owned()
                                )
                            ).transpose()?
                        );
                    }
                    "ConformancePackRuleEvaluationResults" => {
                        builder = builder.set_conformance_pack_rule_evaluation_results(
                            crate::protocol_serde::shape_conformance_pack_rule_evaluation_results_list::de_conformance_pack_rule_evaluation_results_list(tokens)?
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

