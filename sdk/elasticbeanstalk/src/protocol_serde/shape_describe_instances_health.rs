// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_instances_health_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeInstancesHealthOutput, crate::error::DescribeInstancesHealthError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeInstancesHealthError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeInstancesHealthError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ElasticBeanstalkServiceException" => crate::error::DescribeInstancesHealthError::ElasticBeanstalkServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::elastic_beanstalk_service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_elastic_beanstalk_service_exception::de_elastic_beanstalk_service_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DescribeInstancesHealthError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidRequestException" => crate::error::DescribeInstancesHealthError::InvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DescribeInstancesHealthError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeInstancesHealthError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_instances_health_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeInstancesHealthOutput, crate::error::DescribeInstancesHealthError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_instances_health_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_instances_health::de_describe_instances_health(response.body().as_ref(), output).map_err(crate::error::DescribeInstancesHealthError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_instances_health(inp: &[u8], mut builder: crate::output::describe_instances_health_output::Builder) -> Result<crate::output::describe_instances_health_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DescribeInstancesHealthResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DescribeInstancesHealthResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("DescribeInstancesHealthResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected DescribeInstancesHealthResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("InstanceHealthList") /* InstanceHealthList com.amazonaws.elasticbeanstalk.synthetic#DescribeInstancesHealthOutput$InstanceHealthList */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_instance_health_list::de_instance_health_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_instance_health_list(var_1);
            }
            ,
            s if s.matches("RefreshedAt") /* RefreshedAt com.amazonaws.elasticbeanstalk.synthetic#DescribeInstancesHealthOutput$RefreshedAt */ =>  {
                let var_2 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.elasticbeanstalk#RefreshedAt`)"))
                        ?
                    )
                ;
                builder = builder.set_refreshed_at(var_2);
            }
            ,
            s if s.matches("NextToken") /* NextToken com.amazonaws.elasticbeanstalk.synthetic#DescribeInstancesHealthOutput$NextToken */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_next_token(var_3);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected DescribeInstancesHealthResult tag"))
                    };
    Ok(builder)
}

