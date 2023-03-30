// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_reserved_cache_node(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::ReservedCacheNode, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::ReservedCacheNode::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ReservedCacheNodeId") /* ReservedCacheNodeId com.amazonaws.elasticache#ReservedCacheNode$ReservedCacheNodeId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_reserved_cache_node_id(var_1);
            }
            ,
            s if s.matches("ReservedCacheNodesOfferingId") /* ReservedCacheNodesOfferingId com.amazonaws.elasticache#ReservedCacheNode$ReservedCacheNodesOfferingId */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_reserved_cache_nodes_offering_id(var_2);
            }
            ,
            s if s.matches("CacheNodeType") /* CacheNodeType com.amazonaws.elasticache#ReservedCacheNode$CacheNodeType */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_cache_node_type(var_3);
            }
            ,
            s if s.matches("StartTime") /* StartTime com.amazonaws.elasticache#ReservedCacheNode$StartTime */ =>  {
                let var_4 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.elasticache#TStamp`)"))
                        ?
                    )
                ;
                builder = builder.set_start_time(var_4);
            }
            ,
            s if s.matches("Duration") /* Duration com.amazonaws.elasticache#ReservedCacheNode$Duration */ =>  {
                let var_5 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.elasticache#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_duration(var_5);
            }
            ,
            s if s.matches("FixedPrice") /* FixedPrice com.amazonaws.elasticache#ReservedCacheNode$FixedPrice */ =>  {
                let var_6 =
                    Some(
                         {
                            <f64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (double: `com.amazonaws.elasticache#Double`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_fixed_price(var_6);
            }
            ,
            s if s.matches("UsagePrice") /* UsagePrice com.amazonaws.elasticache#ReservedCacheNode$UsagePrice */ =>  {
                let var_7 =
                    Some(
                         {
                            <f64 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (double: `com.amazonaws.elasticache#Double`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_usage_price(var_7);
            }
            ,
            s if s.matches("CacheNodeCount") /* CacheNodeCount com.amazonaws.elasticache#ReservedCacheNode$CacheNodeCount */ =>  {
                let var_8 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.elasticache#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_cache_node_count(var_8);
            }
            ,
            s if s.matches("ProductDescription") /* ProductDescription com.amazonaws.elasticache#ReservedCacheNode$ProductDescription */ =>  {
                let var_9 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_product_description(var_9);
            }
            ,
            s if s.matches("OfferingType") /* OfferingType com.amazonaws.elasticache#ReservedCacheNode$OfferingType */ =>  {
                let var_10 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_offering_type(var_10);
            }
            ,
            s if s.matches("State") /* State com.amazonaws.elasticache#ReservedCacheNode$State */ =>  {
                let var_11 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_11);
            }
            ,
            s if s.matches("RecurringCharges") /* RecurringCharges com.amazonaws.elasticache#ReservedCacheNode$RecurringCharges */ =>  {
                let var_12 =
                    Some(
                        crate::protocol_serde::shape_recurring_charge_list::de_recurring_charge_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_recurring_charges(var_12);
            }
            ,
            s if s.matches("ReservationARN") /* ReservationARN com.amazonaws.elasticache#ReservedCacheNode$ReservationARN */ =>  {
                let var_13 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_reservation_arn(var_13);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

