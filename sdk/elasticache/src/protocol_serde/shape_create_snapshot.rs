// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_snapshot_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateSnapshotOutput, crate::error::CreateSnapshotError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateSnapshotError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateSnapshotError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "CacheClusterNotFound" => crate::error::CreateSnapshotError::CacheClusterNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cache_cluster_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cache_cluster_not_found_fault::de_cache_cluster_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateSnapshotError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidCacheClusterState" => crate::error::CreateSnapshotError::InvalidCacheClusterStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_cache_cluster_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_cache_cluster_state_fault::de_invalid_cache_cluster_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateSnapshotError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterCombination" => crate::error::CreateSnapshotError::InvalidParameterCombinationException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_combination_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_combination_exception::de_invalid_parameter_combination_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateSnapshotError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidParameterValue" => crate::error::CreateSnapshotError::InvalidParameterValueException({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_parameter_value_exception::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_parameter_value_exception::de_invalid_parameter_value_exception_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateSnapshotError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidReplicationGroupState" => crate::error::CreateSnapshotError::InvalidReplicationGroupStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_replication_group_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_replication_group_state_fault::de_invalid_replication_group_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateSnapshotError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ReplicationGroupNotFoundFault" => crate::error::CreateSnapshotError::ReplicationGroupNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::replication_group_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_replication_group_not_found_fault::de_replication_group_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateSnapshotError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "SnapshotAlreadyExistsFault" => crate::error::CreateSnapshotError::SnapshotAlreadyExistsFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::snapshot_already_exists_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_snapshot_already_exists_fault::de_snapshot_already_exists_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateSnapshotError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "SnapshotFeatureNotSupportedFault" => crate::error::CreateSnapshotError::SnapshotFeatureNotSupportedFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::snapshot_feature_not_supported_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_snapshot_feature_not_supported_fault::de_snapshot_feature_not_supported_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateSnapshotError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "SnapshotQuotaExceededFault" => crate::error::CreateSnapshotError::SnapshotQuotaExceededFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::snapshot_quota_exceeded_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_snapshot_quota_exceeded_fault::de_snapshot_quota_exceeded_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateSnapshotError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "TagQuotaPerResourceExceeded" => crate::error::CreateSnapshotError::TagQuotaPerResourceExceeded({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::tag_quota_per_resource_exceeded::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_tag_quota_per_resource_exceeded::de_tag_quota_per_resource_exceeded_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateSnapshotError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreateSnapshotError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_snapshot_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateSnapshotOutput, crate::error::CreateSnapshotError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_snapshot_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_snapshot::de_create_snapshot(response.body().as_ref(), output).map_err(crate::error::CreateSnapshotError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_create_snapshot(inp: &[u8], mut builder: crate::output::create_snapshot_output::Builder) -> Result<crate::output::create_snapshot_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("CreateSnapshotResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected CreateSnapshotResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("CreateSnapshotResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected CreateSnapshotResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("Snapshot") /* Snapshot com.amazonaws.elasticache.synthetic#CreateSnapshotOutput$Snapshot */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_snapshot::de_snapshot(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_snapshot(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected CreateSnapshotResult tag"))
                    };
    Ok(builder)
}

