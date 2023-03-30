// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_cluster_parameters_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeClusterParametersOutput, crate::error::DescribeClusterParametersError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeClusterParametersError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeClusterParametersError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ClusterParameterGroupNotFound" => crate::error::DescribeClusterParametersError::ClusterParameterGroupNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cluster_parameter_group_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cluster_parameter_group_not_found_fault::de_cluster_parameter_group_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::DescribeClusterParametersError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeClusterParametersError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_cluster_parameters_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeClusterParametersOutput, crate::error::DescribeClusterParametersError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_cluster_parameters_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_cluster_parameters::de_describe_cluster_parameters(response.body().as_ref(), output).map_err(crate::error::DescribeClusterParametersError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_cluster_parameters(inp: &[u8], mut builder: crate::output::describe_cluster_parameters_output::Builder) -> Result<crate::output::describe_cluster_parameters_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DescribeClusterParametersResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DescribeClusterParametersResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("DescribeClusterParametersResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected DescribeClusterParametersResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("Parameters") /* Parameters com.amazonaws.redshift.synthetic#DescribeClusterParametersOutput$Parameters */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_parameters_list::de_parameters_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_parameters(var_1);
            }
            ,
            s if s.matches("Marker") /* Marker com.amazonaws.redshift.synthetic#DescribeClusterParametersOutput$Marker */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_marker(var_2);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected DescribeClusterParametersResult tag"))
                    };
    Ok(builder)
}

