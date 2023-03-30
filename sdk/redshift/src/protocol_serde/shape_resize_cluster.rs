// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_resize_cluster_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ResizeClusterOutput, crate::error::ResizeClusterError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::ResizeClusterError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::ResizeClusterError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "ClusterNotFound" => crate::error::ResizeClusterError::ClusterNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::cluster_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_cluster_not_found_fault::de_cluster_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ResizeClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DependentServiceUnavailableFault" => crate::error::ResizeClusterError::DependentServiceUnavailableFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::dependent_service_unavailable_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_dependent_service_unavailable_fault::de_dependent_service_unavailable_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ResizeClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InsufficientClusterCapacity" => crate::error::ResizeClusterError::InsufficientClusterCapacityFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::insufficient_cluster_capacity_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_insufficient_cluster_capacity_fault::de_insufficient_cluster_capacity_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ResizeClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidClusterState" => crate::error::ResizeClusterError::InvalidClusterStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_cluster_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_cluster_state_fault::de_invalid_cluster_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ResizeClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidReservedNodeState" => crate::error::ResizeClusterError::InvalidReservedNodeStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_reserved_node_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_reserved_node_state_fault::de_invalid_reserved_node_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ResizeClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "LimitExceededFault" => crate::error::ResizeClusterError::LimitExceededFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::limit_exceeded_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_limit_exceeded_fault::de_limit_exceeded_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ResizeClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NumberOfNodesPerClusterLimitExceeded" => crate::error::ResizeClusterError::NumberOfNodesPerClusterLimitExceededFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::number_of_nodes_per_cluster_limit_exceeded_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_number_of_nodes_per_cluster_limit_exceeded_fault::de_number_of_nodes_per_cluster_limit_exceeded_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ResizeClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "NumberOfNodesQuotaExceeded" => crate::error::ResizeClusterError::NumberOfNodesQuotaExceededFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::number_of_nodes_quota_exceeded_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_number_of_nodes_quota_exceeded_fault::de_number_of_nodes_quota_exceeded_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ResizeClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ReservedNodeAlreadyExists" => crate::error::ResizeClusterError::ReservedNodeAlreadyExistsFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::reserved_node_already_exists_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_reserved_node_already_exists_fault::de_reserved_node_already_exists_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ResizeClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ReservedNodeAlreadyMigrated" => crate::error::ResizeClusterError::ReservedNodeAlreadyMigratedFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::reserved_node_already_migrated_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_reserved_node_already_migrated_fault::de_reserved_node_already_migrated_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ResizeClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ReservedNodeNotFound" => crate::error::ResizeClusterError::ReservedNodeNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::reserved_node_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_reserved_node_not_found_fault::de_reserved_node_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ResizeClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "ReservedNodeOfferingNotFound" => crate::error::ResizeClusterError::ReservedNodeOfferingNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::reserved_node_offering_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_reserved_node_offering_not_found_fault::de_reserved_node_offering_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ResizeClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnauthorizedOperation" => crate::error::ResizeClusterError::UnauthorizedOperation({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unauthorized_operation::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unauthorized_operation::de_unauthorized_operation_xml_err(response.body().as_ref(), output).map_err(crate::error::ResizeClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedOperation" => crate::error::ResizeClusterError::UnsupportedOperationFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unsupported_operation_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_operation_fault::de_unsupported_operation_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ResizeClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "UnsupportedOptionFault" => crate::error::ResizeClusterError::UnsupportedOptionFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unsupported_option_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_unsupported_option_fault::de_unsupported_option_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::ResizeClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::ResizeClusterError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_resize_cluster_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::ResizeClusterOutput, crate::error::ResizeClusterError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::resize_cluster_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_resize_cluster::de_resize_cluster(response.body().as_ref(), output).map_err(crate::error::ResizeClusterError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_resize_cluster(inp: &[u8], mut builder: crate::output::resize_cluster_output::Builder) -> Result<crate::output::resize_cluster_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("ResizeClusterResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected ResizeClusterResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("ResizeClusterResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected ResizeClusterResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("Cluster") /* Cluster com.amazonaws.redshift.synthetic#ResizeClusterOutput$Cluster */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_cluster::de_cluster(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_cluster(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected ResizeClusterResult tag"))
                    };
    Ok(builder)
}

