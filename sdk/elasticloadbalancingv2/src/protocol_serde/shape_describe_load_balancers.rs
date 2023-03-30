// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_load_balancers_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeLoadBalancersOutput, crate::error::DescribeLoadBalancersError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeLoadBalancersError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeLoadBalancersError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "LoadBalancerNotFound" => crate::error::DescribeLoadBalancersError::LoadBalancerNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::load_balancer_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_load_balancer_not_found_exception::de_load_balancer_not_found_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DescribeLoadBalancersError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeLoadBalancersError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_load_balancers_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeLoadBalancersOutput, crate::error::DescribeLoadBalancersError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_load_balancers_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_load_balancers::de_describe_load_balancers(response.body().as_ref(), output).map_err(crate::error::DescribeLoadBalancersError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_load_balancers(inp: &[u8], mut builder: crate::output::describe_load_balancers_output::Builder) -> Result<crate::output::describe_load_balancers_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DescribeLoadBalancersResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DescribeLoadBalancersResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("DescribeLoadBalancersResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected DescribeLoadBalancersResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("LoadBalancers") /* LoadBalancers com.amazonaws.elasticloadbalancingv2.synthetic#DescribeLoadBalancersOutput$LoadBalancers */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_load_balancers::de_load_balancers(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_load_balancers(var_1);
            }
            ,
            s if s.matches("NextMarker") /* NextMarker com.amazonaws.elasticloadbalancingv2.synthetic#DescribeLoadBalancersOutput$NextMarker */ =>  {
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
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected DescribeLoadBalancersResult tag"))
                    };
    Ok(builder)
}

