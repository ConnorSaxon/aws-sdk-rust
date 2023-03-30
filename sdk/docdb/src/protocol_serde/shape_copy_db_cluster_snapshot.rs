// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_copy_db_cluster_snapshot_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CopyDbClusterSnapshotOutput, crate::error::CopyDBClusterSnapshotError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CopyDBClusterSnapshotError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CopyDBClusterSnapshotError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "DBClusterSnapshotAlreadyExistsFault" => crate::error::CopyDBClusterSnapshotError::DbClusterSnapshotAlreadyExistsFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::db_cluster_snapshot_already_exists_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_cluster_snapshot_already_exists_fault::de_db_cluster_snapshot_already_exists_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CopyDBClusterSnapshotError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DBClusterSnapshotNotFoundFault" => crate::error::CopyDBClusterSnapshotError::DbClusterSnapshotNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::db_cluster_snapshot_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_cluster_snapshot_not_found_fault::de_db_cluster_snapshot_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CopyDBClusterSnapshotError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidDBClusterSnapshotStateFault" => crate::error::CopyDBClusterSnapshotError::InvalidDbClusterSnapshotStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_db_cluster_snapshot_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_db_cluster_snapshot_state_fault::de_invalid_db_cluster_snapshot_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CopyDBClusterSnapshotError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidDBClusterStateFault" => crate::error::CopyDBClusterSnapshotError::InvalidDbClusterStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_db_cluster_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_db_cluster_state_fault::de_invalid_db_cluster_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CopyDBClusterSnapshotError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "KMSKeyNotAccessibleFault" => crate::error::CopyDBClusterSnapshotError::KmsKeyNotAccessibleFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::kms_key_not_accessible_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_kms_key_not_accessible_fault::de_kms_key_not_accessible_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CopyDBClusterSnapshotError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "SnapshotQuotaExceeded" => crate::error::CopyDBClusterSnapshotError::SnapshotQuotaExceededFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::snapshot_quota_exceeded_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_snapshot_quota_exceeded_fault::de_snapshot_quota_exceeded_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CopyDBClusterSnapshotError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CopyDBClusterSnapshotError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_copy_db_cluster_snapshot_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CopyDbClusterSnapshotOutput, crate::error::CopyDBClusterSnapshotError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::copy_db_cluster_snapshot_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_copy_db_cluster_snapshot::de_copy_db_cluster_snapshot(response.body().as_ref(), output).map_err(crate::error::CopyDBClusterSnapshotError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_copy_db_cluster_snapshot(inp: &[u8], mut builder: crate::output::copy_db_cluster_snapshot_output::Builder) -> Result<crate::output::copy_db_cluster_snapshot_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("CopyDBClusterSnapshotResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected CopyDBClusterSnapshotResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("CopyDBClusterSnapshotResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected CopyDBClusterSnapshotResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("DBClusterSnapshot") /* DBClusterSnapshot com.amazonaws.docdb.synthetic#CopyDBClusterSnapshotOutput$DBClusterSnapshot */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_db_cluster_snapshot::de_db_cluster_snapshot(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_db_cluster_snapshot(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected CopyDBClusterSnapshotResult tag"))
                    };
    Ok(builder)
}

