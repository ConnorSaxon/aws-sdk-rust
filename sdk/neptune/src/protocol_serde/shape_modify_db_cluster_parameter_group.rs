// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_db_cluster_parameter_group_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ModifyDbClusterParameterGroupOutput, crate::error::ModifyDBClusterParameterGroupError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ModifyDBClusterParameterGroupError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ModifyDBClusterParameterGroupError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "DBParameterGroupNotFound" => crate::error::ModifyDBClusterParameterGroupError::DbParameterGroupNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::db_parameter_group_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_parameter_group_not_found_fault::de_db_parameter_group_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ModifyDBClusterParameterGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidDBParameterGroupState" => crate::error::ModifyDBClusterParameterGroupError::InvalidDbParameterGroupStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_db_parameter_group_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_db_parameter_group_state_fault::de_invalid_db_parameter_group_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ModifyDBClusterParameterGroupError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ModifyDBClusterParameterGroupError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_db_cluster_parameter_group_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ModifyDbClusterParameterGroupOutput, crate::error::ModifyDBClusterParameterGroupError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::modify_db_cluster_parameter_group_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_modify_db_cluster_parameter_group::de_modify_db_cluster_parameter_group(response.body().as_ref(), output).map_err(crate::error::ModifyDBClusterParameterGroupError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_modify_db_cluster_parameter_group(inp: &[u8], mut builder: crate::output::modify_db_cluster_parameter_group_output::Builder) -> Result<crate::output::modify_db_cluster_parameter_group_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("ModifyDBClusterParameterGroupResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected ModifyDBClusterParameterGroupResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("ModifyDBClusterParameterGroupResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected ModifyDBClusterParameterGroupResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("DBClusterParameterGroupName") /* DBClusterParameterGroupName com.amazonaws.neptune.synthetic#ModifyDBClusterParameterGroupOutput$DBClusterParameterGroupName */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_cluster_parameter_group_name(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected ModifyDBClusterParameterGroupResult tag"))
                    };
    Ok(builder)
}

