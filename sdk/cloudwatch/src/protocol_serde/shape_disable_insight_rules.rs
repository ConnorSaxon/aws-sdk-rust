// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_disable_insight_rules_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DisableInsightRulesOutput, crate::error::DisableInsightRulesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DisableInsightRulesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DisableInsightRulesError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "InvalidParameterValue" => crate::error::DisableInsightRulesError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_value_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DisableInsightRulesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "MissingParameter" => crate::error::DisableInsightRulesError::MissingRequiredParameterException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::missing_required_parameter_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_missing_required_parameter_exception::de_missing_required_parameter_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DisableInsightRulesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DisableInsightRulesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_disable_insight_rules_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DisableInsightRulesOutput, crate::error::DisableInsightRulesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::disable_insight_rules_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_disable_insight_rules::de_disable_insight_rules(response.body().as_ref(), output).map_err(crate::error::DisableInsightRulesError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_disable_insight_rules(inp: &[u8], mut builder: crate::output::disable_insight_rules_output::Builder) -> Result<crate::output::disable_insight_rules_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DisableInsightRulesResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DisableInsightRulesResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("DisableInsightRulesResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected DisableInsightRulesResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("Failures") /* Failures com.amazonaws.cloudwatch.synthetic#DisableInsightRulesOutput$Failures */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_batch_failures::de_batch_failures(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_failures(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected DisableInsightRulesResult tag"))
                    };
    Ok(builder)
}

