// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_pending_modified_values(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::PendingModifiedValues, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::PendingModifiedValues::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("NumCacheNodes") /* NumCacheNodes com.amazonaws.elasticache#PendingModifiedValues$NumCacheNodes */ =>  {
                let var_1 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.elasticache#IntegerOptional`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_num_cache_nodes(var_1);
            }
            ,
            s if s.matches("CacheNodeIdsToRemove") /* CacheNodeIdsToRemove com.amazonaws.elasticache#PendingModifiedValues$CacheNodeIdsToRemove */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_cache_node_ids_list::de_cache_node_ids_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_cache_node_ids_to_remove(var_2);
            }
            ,
            s if s.matches("EngineVersion") /* EngineVersion com.amazonaws.elasticache#PendingModifiedValues$EngineVersion */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_engine_version(var_3);
            }
            ,
            s if s.matches("CacheNodeType") /* CacheNodeType com.amazonaws.elasticache#PendingModifiedValues$CacheNodeType */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_cache_node_type(var_4);
            }
            ,
            s if s.matches("AuthTokenStatus") /* AuthTokenStatus com.amazonaws.elasticache#PendingModifiedValues$AuthTokenStatus */ =>  {
                let var_5 =
                    Some(
                        Result::<crate::model::AuthTokenUpdateStatus, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::AuthTokenUpdateStatus::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_auth_token_status(var_5);
            }
            ,
            s if s.matches("LogDeliveryConfigurations") /* LogDeliveryConfigurations com.amazonaws.elasticache#PendingModifiedValues$LogDeliveryConfigurations */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_pending_log_delivery_configuration_list::de_pending_log_delivery_configuration_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_log_delivery_configurations(var_6);
            }
            ,
            s if s.matches("TransitEncryptionEnabled") /* TransitEncryptionEnabled com.amazonaws.elasticache#PendingModifiedValues$TransitEncryptionEnabled */ =>  {
                let var_7 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.elasticache#BooleanOptional`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_transit_encryption_enabled(var_7);
            }
            ,
            s if s.matches("TransitEncryptionMode") /* TransitEncryptionMode com.amazonaws.elasticache#PendingModifiedValues$TransitEncryptionMode */ =>  {
                let var_8 =
                    Some(
                        Result::<crate::model::TransitEncryptionMode, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::TransitEncryptionMode::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_transit_encryption_mode(var_8);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

