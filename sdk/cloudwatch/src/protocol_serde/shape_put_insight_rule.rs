// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_put_insight_rule_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PutInsightRuleOutput, crate::error::PutInsightRuleError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::PutInsightRuleError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::PutInsightRuleError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidParameterValue" => crate::error::PutInsightRuleError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_value_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::PutInsightRuleError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceededException" => crate::error::PutInsightRuleError::LimitExceededException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_exception::de_limit_exceeded_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::PutInsightRuleError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "MissingParameter" => crate::error::PutInsightRuleError::MissingRequiredParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::missing_required_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_missing_required_parameter_exception::de_missing_required_parameter_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::PutInsightRuleError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::PutInsightRuleError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_put_insight_rule_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::PutInsightRuleOutput, crate::error::PutInsightRuleError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::put_insight_rule_output::Builder::default();
        let _ = response;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

