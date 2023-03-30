// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_ssl_policies_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeSslPoliciesOutput, crate::error::DescribeSSLPoliciesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeSSLPoliciesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeSSLPoliciesError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "SSLPolicyNotFound" => crate::error::DescribeSSLPoliciesError::SslPolicyNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::ssl_policy_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_ssl_policy_not_found_exception::de_ssl_policy_not_found_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DescribeSSLPoliciesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeSSLPoliciesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_ssl_policies_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeSslPoliciesOutput, crate::error::DescribeSSLPoliciesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_ssl_policies_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_ssl_policies::de_describe_ssl_policies(response.body().as_ref(), output).map_err(crate::error::DescribeSSLPoliciesError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_ssl_policies(inp: &[u8], mut builder: crate::output::describe_ssl_policies_output::Builder) -> Result<crate::output::describe_ssl_policies_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DescribeSSLPoliciesResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DescribeSSLPoliciesResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("DescribeSSLPoliciesResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected DescribeSSLPoliciesResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("SslPolicies") /* SslPolicies com.amazonaws.elasticloadbalancingv2.synthetic#DescribeSSLPoliciesOutput$SslPolicies */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_ssl_policies::de_ssl_policies(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ssl_policies(var_1);
            }
            ,
            s if s.matches("NextMarker") /* NextMarker com.amazonaws.elasticloadbalancingv2.synthetic#DescribeSSLPoliciesOutput$NextMarker */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_marker(var_2);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected DescribeSSLPoliciesResult tag"))
                    };
    Ok(builder)
}

