// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_approval_rule_template_input(input: &crate::input::DeleteApprovalRuleTemplateInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    let mut object = aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_approval_rule_template_input::ser_delete_approval_rule_template_input(&mut object, input)?;
    object.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_approval_rule_template_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteApprovalRuleTemplateOutput, crate::error::DeleteApprovalRuleTemplateError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DeleteApprovalRuleTemplateError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DeleteApprovalRuleTemplateError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ApprovalRuleTemplateInUseException" => crate::error::DeleteApprovalRuleTemplateError::ApprovalRuleTemplateInUseException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::approval_rule_template_in_use_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_approval_rule_template_in_use_exception::de_approval_rule_template_in_use_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteApprovalRuleTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ApprovalRuleTemplateNameRequiredException" => crate::error::DeleteApprovalRuleTemplateError::ApprovalRuleTemplateNameRequiredException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::approval_rule_template_name_required_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_approval_rule_template_name_required_exception::de_approval_rule_template_name_required_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteApprovalRuleTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidApprovalRuleTemplateNameException" => crate::error::DeleteApprovalRuleTemplateError::InvalidApprovalRuleTemplateNameException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_approval_rule_template_name_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_approval_rule_template_name_exception::de_invalid_approval_rule_template_name_exception_json_err(response.body().as_ref(), output).map_err(crate::error::DeleteApprovalRuleTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DeleteApprovalRuleTemplateError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_approval_rule_template_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DeleteApprovalRuleTemplateOutput, crate::error::DeleteApprovalRuleTemplateError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::delete_approval_rule_template_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_delete_approval_rule_template::de_delete_approval_rule_template(response.body().as_ref(), output).map_err(crate::error::DeleteApprovalRuleTemplateError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_delete_approval_rule_template(value: &[u8], mut builder: crate::output::delete_approval_rule_template_output::Builder) -> Result<crate::output::delete_approval_rule_template_output::Builder, aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
                        let tokens = &mut tokens_owned;
                        aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                    Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                match key.to_unescaped()?.as_ref() {
                    "approvalRuleTemplateId" => {
                        builder = builder.set_approval_rule_template_id(
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

