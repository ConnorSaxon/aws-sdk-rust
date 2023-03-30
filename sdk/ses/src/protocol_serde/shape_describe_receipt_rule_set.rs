// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_receipt_rule_set_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeReceiptRuleSetOutput, crate::error::DescribeReceiptRuleSetError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeReceiptRuleSetError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeReceiptRuleSetError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "RuleSetDoesNotExist" => crate::error::DescribeReceiptRuleSetError::RuleSetDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::rule_set_does_not_exist_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_rule_set_does_not_exist_exception::de_rule_set_does_not_exist_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DescribeReceiptRuleSetError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeReceiptRuleSetError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_receipt_rule_set_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeReceiptRuleSetOutput, crate::error::DescribeReceiptRuleSetError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_receipt_rule_set_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_receipt_rule_set::de_describe_receipt_rule_set(response.body().as_ref(), output).map_err(crate::error::DescribeReceiptRuleSetError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_receipt_rule_set(inp: &[u8], mut builder: crate::output::describe_receipt_rule_set_output::Builder) -> Result<crate::output::describe_receipt_rule_set_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DescribeReceiptRuleSetResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DescribeReceiptRuleSetResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("DescribeReceiptRuleSetResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected DescribeReceiptRuleSetResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("Metadata") /* Metadata com.amazonaws.ses.synthetic#DescribeReceiptRuleSetOutput$Metadata */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_receipt_rule_set_metadata::de_receipt_rule_set_metadata(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_metadata(var_1);
            }
            ,
            s if s.matches("Rules") /* Rules com.amazonaws.ses.synthetic#DescribeReceiptRuleSetOutput$Rules */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_receipt_rules_list::de_receipt_rules_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_rules(var_2);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected DescribeReceiptRuleSetResult tag"))
                    };
    Ok(builder)
}

