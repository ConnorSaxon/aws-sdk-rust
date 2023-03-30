// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_environment_health_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeEnvironmentHealthOutput, crate::error::DescribeEnvironmentHealthError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeEnvironmentHealthError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeEnvironmentHealthError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ElasticBeanstalkServiceException" => crate::error::DescribeEnvironmentHealthError::ElasticBeanstalkServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::elastic_beanstalk_service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_elastic_beanstalk_service_exception::de_elastic_beanstalk_service_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DescribeEnvironmentHealthError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidRequestException" => crate::error::DescribeEnvironmentHealthError::InvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_request_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DescribeEnvironmentHealthError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeEnvironmentHealthError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_environment_health_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeEnvironmentHealthOutput, crate::error::DescribeEnvironmentHealthError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_environment_health_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_environment_health::de_describe_environment_health(response.body().as_ref(), output).map_err(crate::error::DescribeEnvironmentHealthError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_environment_health(inp: &[u8], mut builder: crate::output::describe_environment_health_output::Builder) -> Result<crate::output::describe_environment_health_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DescribeEnvironmentHealthResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DescribeEnvironmentHealthResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("DescribeEnvironmentHealthResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected DescribeEnvironmentHealthResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("EnvironmentName") /* EnvironmentName com.amazonaws.elasticbeanstalk.synthetic#DescribeEnvironmentHealthOutput$EnvironmentName */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_environment_name(var_1);
            }
            ,
            s if s.matches("HealthStatus") /* HealthStatus com.amazonaws.elasticbeanstalk.synthetic#DescribeEnvironmentHealthOutput$HealthStatus */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_health_status(var_2);
            }
            ,
            s if s.matches("Status") /* Status com.amazonaws.elasticbeanstalk.synthetic#DescribeEnvironmentHealthOutput$Status */ =>  {
                let var_3 =
                    Some(
                        Result::<crate::model::EnvironmentHealth, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::EnvironmentHealth::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_status(var_3);
            }
            ,
            s if s.matches("Color") /* Color com.amazonaws.elasticbeanstalk.synthetic#DescribeEnvironmentHealthOutput$Color */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_color(var_4);
            }
            ,
            s if s.matches("Causes") /* Causes com.amazonaws.elasticbeanstalk.synthetic#DescribeEnvironmentHealthOutput$Causes */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_causes::de_causes(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_causes(var_5);
            }
            ,
            s if s.matches("ApplicationMetrics") /* ApplicationMetrics com.amazonaws.elasticbeanstalk.synthetic#DescribeEnvironmentHealthOutput$ApplicationMetrics */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_application_metrics::de_application_metrics(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_application_metrics(var_6);
            }
            ,
            s if s.matches("InstancesHealth") /* InstancesHealth com.amazonaws.elasticbeanstalk.synthetic#DescribeEnvironmentHealthOutput$InstancesHealth */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_instance_health_summary::de_instance_health_summary(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_instances_health(var_7);
            }
            ,
            s if s.matches("RefreshedAt") /* RefreshedAt com.amazonaws.elasticbeanstalk.synthetic#DescribeEnvironmentHealthOutput$RefreshedAt */ =>  {
                let var_8 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.elasticbeanstalk#RefreshedAt`)"))
                        ?
                    )
                ;
                builder = builder.set_refreshed_at(var_8);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected DescribeEnvironmentHealthResult tag"))
                    };
    Ok(builder)
}

