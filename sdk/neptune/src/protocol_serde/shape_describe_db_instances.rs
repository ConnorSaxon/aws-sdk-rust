// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_db_instances_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeDbInstancesOutput, crate::error::DescribeDBInstancesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::DescribeDBInstancesError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::DescribeDBInstancesError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "DBInstanceNotFound" => crate::error::DescribeDBInstancesError::DbInstanceNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::db_instance_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_instance_not_found_fault::de_db_instance_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::DescribeDBInstancesError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::DescribeDBInstancesError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_db_instances_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::DescribeDbInstancesOutput, crate::error::DescribeDBInstancesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::describe_db_instances_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_describe_db_instances::de_describe_db_instances(response.body().as_ref(), output).map_err(crate::error::DescribeDBInstancesError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_describe_db_instances(inp: &[u8], mut builder: crate::output::describe_db_instances_output::Builder) -> Result<crate::output::describe_db_instances_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("DescribeDBInstancesResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected DescribeDBInstancesResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("DescribeDBInstancesResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected DescribeDBInstancesResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("Marker") /* Marker com.amazonaws.neptune.synthetic#DescribeDBInstancesOutput$Marker */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_marker(var_1);
            }
            ,
            s if s.matches("DBInstances") /* DBInstances com.amazonaws.neptune.synthetic#DescribeDBInstancesOutput$DBInstances */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_db_instance_list::de_db_instance_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_db_instances(var_2);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected DescribeDBInstancesResult tag"))
                    };
    Ok(builder)
}

