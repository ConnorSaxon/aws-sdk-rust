// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_platform_version_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribePlatformVersionOutput, crate::error::DescribePlatformVersionError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribePlatformVersionError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribePlatformVersionError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ElasticBeanstalkServiceException" => crate::error::DescribePlatformVersionError::ElasticBeanstalkServiceException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::elastic_beanstalk_service_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_elastic_beanstalk_service_exception::de_elastic_beanstalk_service_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DescribePlatformVersionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InsufficientPrivilegesException" => crate::error::DescribePlatformVersionError::InsufficientPrivilegesException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::insufficient_privileges_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_insufficient_privileges_exception::de_insufficient_privileges_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::DescribePlatformVersionError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribePlatformVersionError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_platform_version_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribePlatformVersionOutput, crate::error::DescribePlatformVersionError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_platform_version_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_platform_version::de_describe_platform_version(response.body().as_ref(), output).map_err(crate::error::DescribePlatformVersionError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_platform_version(inp: &[u8], mut builder: crate::output::describe_platform_version_output::Builder) -> Result<crate::output::describe_platform_version_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DescribePlatformVersionResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DescribePlatformVersionResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("DescribePlatformVersionResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected DescribePlatformVersionResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("PlatformDescription") /* PlatformDescription com.amazonaws.elasticbeanstalk.synthetic#DescribePlatformVersionOutput$PlatformDescription */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_platform_description::de_platform_description(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_platform_description(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected DescribePlatformVersionResult tag"))
                    };
    Ok(builder)
}

