// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_copy_db_cluster_parameter_group_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CopyDbClusterParameterGroupOutput, crate::error::CopyDBClusterParameterGroupError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CopyDBClusterParameterGroupError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CopyDBClusterParameterGroupError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "DBParameterGroupAlreadyExists" => crate::error::CopyDBClusterParameterGroupError::DbParameterGroupAlreadyExistsFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::db_parameter_group_already_exists_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_parameter_group_already_exists_fault::de_db_parameter_group_already_exists_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CopyDBClusterParameterGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DBParameterGroupNotFound" => crate::error::CopyDBClusterParameterGroupError::DbParameterGroupNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::db_parameter_group_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_parameter_group_not_found_fault::de_db_parameter_group_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CopyDBClusterParameterGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DBParameterGroupQuotaExceeded" => crate::error::CopyDBClusterParameterGroupError::DbParameterGroupQuotaExceededFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::db_parameter_group_quota_exceeded_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_parameter_group_quota_exceeded_fault::de_db_parameter_group_quota_exceeded_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CopyDBClusterParameterGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CopyDBClusterParameterGroupError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_copy_db_cluster_parameter_group_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CopyDbClusterParameterGroupOutput, crate::error::CopyDBClusterParameterGroupError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::copy_db_cluster_parameter_group_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_copy_db_cluster_parameter_group::de_copy_db_cluster_parameter_group(response.body().as_ref(), output).map_err(crate::error::CopyDBClusterParameterGroupError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_copy_db_cluster_parameter_group(inp: &[u8], mut builder: crate::output::copy_db_cluster_parameter_group_output::Builder) -> Result<crate::output::copy_db_cluster_parameter_group_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("CopyDBClusterParameterGroupResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected CopyDBClusterParameterGroupResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("CopyDBClusterParameterGroupResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected CopyDBClusterParameterGroupResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("DBClusterParameterGroup") /* DBClusterParameterGroup com.amazonaws.neptune.synthetic#CopyDBClusterParameterGroupOutput$DBClusterParameterGroup */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_db_cluster_parameter_group::de_db_cluster_parameter_group(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_db_cluster_parameter_group(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected CopyDBClusterParameterGroupResult tag"))
                    };
    Ok(builder)
}

