// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_db_cluster_http_error(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateDbClusterOutput, crate::error::CreateDBClusterError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(response).map_err(crate::error::CreateDBClusterError::unhandled)?;
    generic_builder = aws_http::request_id::apply_request_id(generic_builder, response.headers());
    let generic = generic_builder.build();
    let error_code = match generic.code() {
                                Some(code) => code,
                                None => return Err(crate::error::CreateDBClusterError::unhandled(generic))
                            };
    
                            let _error_message = generic.message().map(|msg|msg.to_owned());
    Err(match error_code {
        "DBClusterAlreadyExistsFault" => crate::error::CreateDBClusterError::DbClusterAlreadyExistsFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::db_cluster_already_exists_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_cluster_already_exists_fault::de_db_cluster_already_exists_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateDBClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DBClusterNotFoundFault" => crate::error::CreateDBClusterError::DbClusterNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::db_cluster_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_cluster_not_found_fault::de_db_cluster_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateDBClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DBClusterParameterGroupNotFound" => crate::error::CreateDBClusterError::DbClusterParameterGroupNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::db_cluster_parameter_group_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_cluster_parameter_group_not_found_fault::de_db_cluster_parameter_group_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateDBClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DBClusterQuotaExceededFault" => crate::error::CreateDBClusterError::DbClusterQuotaExceededFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::db_cluster_quota_exceeded_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_cluster_quota_exceeded_fault::de_db_cluster_quota_exceeded_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateDBClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DBInstanceNotFound" => crate::error::CreateDBClusterError::DbInstanceNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::db_instance_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_instance_not_found_fault::de_db_instance_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateDBClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DBSubnetGroupDoesNotCoverEnoughAZs" => crate::error::CreateDBClusterError::DbSubnetGroupDoesNotCoverEnoughAZs({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::db_subnet_group_does_not_cover_enough_a_zs::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_subnet_group_does_not_cover_enough_a_zs::de_db_subnet_group_does_not_cover_enough_a_zs_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateDBClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "DBSubnetGroupNotFoundFault" => crate::error::CreateDBClusterError::DbSubnetGroupNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::db_subnet_group_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_db_subnet_group_not_found_fault::de_db_subnet_group_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateDBClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "GlobalClusterNotFoundFault" => crate::error::CreateDBClusterError::GlobalClusterNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::global_cluster_not_found_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_global_cluster_not_found_fault::de_global_cluster_not_found_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateDBClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InsufficientStorageClusterCapacity" => crate::error::CreateDBClusterError::InsufficientStorageClusterCapacityFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::insufficient_storage_cluster_capacity_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_insufficient_storage_cluster_capacity_fault::de_insufficient_storage_cluster_capacity_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateDBClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidDBClusterStateFault" => crate::error::CreateDBClusterError::InvalidDbClusterStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_db_cluster_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_db_cluster_state_fault::de_invalid_db_cluster_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateDBClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidDBInstanceState" => crate::error::CreateDBClusterError::InvalidDbInstanceStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_db_instance_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_db_instance_state_fault::de_invalid_db_instance_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateDBClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidDBSubnetGroupStateFault" => crate::error::CreateDBClusterError::InvalidDbSubnetGroupStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_db_subnet_group_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_db_subnet_group_state_fault::de_invalid_db_subnet_group_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateDBClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidGlobalClusterStateFault" => crate::error::CreateDBClusterError::InvalidGlobalClusterStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_global_cluster_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_global_cluster_state_fault::de_invalid_global_cluster_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateDBClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidSubnet" => crate::error::CreateDBClusterError::InvalidSubnet({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_subnet::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_subnet::de_invalid_subnet_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateDBClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "InvalidVPCNetworkStateFault" => crate::error::CreateDBClusterError::InvalidVpcNetworkStateFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_vpc_network_state_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_invalid_vpc_network_state_fault::de_invalid_vpc_network_state_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateDBClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "KMSKeyNotAccessibleFault" => crate::error::CreateDBClusterError::KmsKeyNotAccessibleFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::kms_key_not_accessible_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_kms_key_not_accessible_fault::de_kms_key_not_accessible_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateDBClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        "StorageQuotaExceeded" => crate::error::CreateDBClusterError::StorageQuotaExceededFault({
            #[allow(unused_mut)]
            let mut tmp =
                 {
                    #[allow(unused_mut)]
                    let mut output = crate::error::storage_quota_exceeded_fault::Builder::default();
                    let _ = response;
                    output = crate::protocol_serde::shape_storage_quota_exceeded_fault::de_storage_quota_exceeded_fault_xml_err(response.body().as_ref(), output).map_err(crate::error::CreateDBClusterError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                }
            ;
            if tmp.message.is_none() {
                                                        tmp.message = _error_message;
                                                    }
            tmp
        }),
        _ => crate::error::CreateDBClusterError::generic(generic)
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_db_cluster_http_response(response: &http::Response<bytes::Bytes>) -> std::result::Result<crate::output::CreateDbClusterOutput, crate::error::CreateDBClusterError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::create_db_cluster_output::Builder::default();
        let _ = response;
        output = crate::protocol_serde::shape_create_db_cluster::de_create_db_cluster(response.body().as_ref(), output).map_err(crate::error::CreateDBClusterError::unhandled)?;
        output._set_request_id(aws_http::request_id::RequestId::request_id(response).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_create_db_cluster(inp: &[u8], mut builder: crate::output::create_db_cluster_output::Builder) -> Result<crate::output::create_db_cluster_output::Builder, aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = aws_smithy_xml::decode::Document::try_from(inp)?;
    
                        #[allow(unused_mut)]
                        let mut decoder = doc.root_element()?;
                        #[allow(unused_variables)]
                        let start_el = decoder.start_el();
    if !(start_el.matches("CreateDBClusterResponse")) {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid root, expected CreateDBClusterResponse got {:?}", start_el)))
                    }
                    if let Some(mut result_tag) = decoder.next_tag() {
                        let start_el = result_tag.start_el();
                        if !(start_el.matches("CreateDBClusterResult")) {
                            return Err(aws_smithy_xml::decode::XmlDecodeError::custom(format!("invalid result, expected CreateDBClusterResult got {:?}", start_el)))
                        }
    while let Some(mut tag) = result_tag.next_tag() {
        match tag.start_el() {
            s if s.matches("DBCluster") /* DBCluster com.amazonaws.neptune.synthetic#CreateDBClusterOutput$DBCluster */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_db_cluster::de_db_cluster(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_db_cluster(var_1);
            }
            ,
            _ => {}
        }
    }
    } else {
                        return Err(aws_smithy_xml::decode::XmlDecodeError::custom("expected CreateDBClusterResult tag"))
                    };
    Ok(builder)
}

