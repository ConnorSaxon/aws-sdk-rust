// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_batch_modify_cluster_snapshots_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::BatchModifyClusterSnapshotsOutput, crate::error::BatchModifyClusterSnapshotsError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::BatchModifyClusterSnapshotsError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::BatchModifyClusterSnapshotsError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "BatchModifyClusterSnapshotsLimitExceededFault" => crate::error::BatchModifyClusterSnapshotsError::BatchModifyClusterSnapshotsLimitExceededFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::batch_modify_cluster_snapshots_limit_exceeded_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_batch_modify_cluster_snapshots_limit_exceeded_fault::de_batch_modify_cluster_snapshots_limit_exceeded_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::BatchModifyClusterSnapshotsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidRetentionPeriodFault" => crate::error::BatchModifyClusterSnapshotsError::InvalidRetentionPeriodFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_retention_period_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_retention_period_fault::de_invalid_retention_period_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::BatchModifyClusterSnapshotsError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::BatchModifyClusterSnapshotsError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_batch_modify_cluster_snapshots_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::BatchModifyClusterSnapshotsOutput, crate::error::BatchModifyClusterSnapshotsError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::batch_modify_cluster_snapshots_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_batch_modify_cluster_snapshots::de_batch_modify_cluster_snapshots(response.body().as_ref(), output).map_err(crate::error::BatchModifyClusterSnapshotsError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_batch_modify_cluster_snapshots(inp: &[u8], mut builder: crate::output::batch_modify_cluster_snapshots_output::Builder) -> Result<crate::output::batch_modify_cluster_snapshots_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("BatchModifyClusterSnapshotsResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected BatchModifyClusterSnapshotsResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("BatchModifyClusterSnapshotsResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected BatchModifyClusterSnapshotsResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("Resources") /* Resources com.amazonaws.redshift.synthetic#BatchModifyClusterSnapshotsOutput$Resources */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_snapshot_identifier_list::de_snapshot_identifier_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_resources(var_1);
            }
            ,
            s if s.matches("Errors") /* Errors com.amazonaws.redshift.synthetic#BatchModifyClusterSnapshotsOutput$Errors */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_batch_snapshot_operation_errors::de_batch_snapshot_operation_errors(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_errors(var_2);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected BatchModifyClusterSnapshotsResult tag"))
                    };
    Ok(builder)
}

