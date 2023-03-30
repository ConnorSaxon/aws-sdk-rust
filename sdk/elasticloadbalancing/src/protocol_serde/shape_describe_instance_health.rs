// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_instance_health_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeInstanceHealthOutput, crate::error::DescribeInstanceHealthError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeInstanceHealthError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeInstanceHealthError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "LoadBalancerNotFound" => crate::error::DescribeInstanceHealthError::AccessPointNotFoundException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::access_point_not_found_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_access_point_not_found_exception::de_access_point_not_found_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DescribeInstanceHealthError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidInstance" => crate::error::DescribeInstanceHealthError::InvalidEndPointException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_end_point_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_end_point_exception::de_invalid_end_point_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DescribeInstanceHealthError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeInstanceHealthError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_instance_health_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeInstanceHealthOutput, crate::error::DescribeInstanceHealthError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_instance_health_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_instance_health::de_describe_instance_health(response.body().as_ref(), output).map_err(crate::error::DescribeInstanceHealthError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_instance_health(inp: &[u8], mut builder: crate::output::describe_instance_health_output::Builder) -> Result<crate::output::describe_instance_health_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DescribeInstanceHealthResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DescribeInstanceHealthResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("DescribeInstanceHealthResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected DescribeInstanceHealthResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("InstanceStates") /* InstanceStates com.amazonaws.elasticloadbalancing.synthetic#DescribeInstanceHealthOutput$InstanceStates */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_instance_states::de_instance_states(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_instance_states(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected DescribeInstanceHealthResult tag"))
                    };
    Ok(builder)
}

