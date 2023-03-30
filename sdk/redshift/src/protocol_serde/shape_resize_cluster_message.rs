// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_resize_cluster_message(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::ResizeClusterMessage) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ClusterIdentifier");
    if let Some(var_2) = &input.cluster_identifier {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ClusterType");
    if let Some(var_4) = &input.cluster_type {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("NodeType");
    if let Some(var_6) = &input.node_type {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("NumberOfNodes");
    if let Some(var_8) = &input.number_of_nodes {
        scope_7.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_8).into()));
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("Classic");
    if let Some(var_10) = &input.classic {
        scope_9.boolean(*var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("ReservedNodeId");
    if let Some(var_12) = &input.reserved_node_id {
        scope_11.string(var_12);
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("TargetReservedNodeOfferingId");
    if let Some(var_14) = &input.target_reserved_node_offering_id {
        scope_13.string(var_14);
    }
    Ok(())
}

pub fn de_resize_cluster_message(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::ResizeClusterMessage, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::ResizeClusterMessage::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ClusterIdentifier") /* ClusterIdentifier com.amazonaws.redshift#ResizeClusterMessage$ClusterIdentifier */ =>  {
                let var_15 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_cluster_identifier(var_15);
            }
            ,
            s if s.matches("ClusterType") /* ClusterType com.amazonaws.redshift#ResizeClusterMessage$ClusterType */ =>  {
                let var_16 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_cluster_type(var_16);
            }
            ,
            s if s.matches("NodeType") /* NodeType com.amazonaws.redshift#ResizeClusterMessage$NodeType */ =>  {
                let var_17 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_node_type(var_17);
            }
            ,
            s if s.matches("NumberOfNodes") /* NumberOfNodes com.amazonaws.redshift#ResizeClusterMessage$NumberOfNodes */ =>  {
                let var_18 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.redshift#IntegerOptional`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_number_of_nodes(var_18);
            }
            ,
            s if s.matches("Classic") /* Classic com.amazonaws.redshift#ResizeClusterMessage$Classic */ =>  {
                let var_19 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.redshift#BooleanOptional`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_classic(var_19);
            }
            ,
            s if s.matches("ReservedNodeId") /* ReservedNodeId com.amazonaws.redshift#ResizeClusterMessage$ReservedNodeId */ =>  {
                let var_20 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_reserved_node_id(var_20);
            }
            ,
            s if s.matches("TargetReservedNodeOfferingId") /* TargetReservedNodeOfferingId com.amazonaws.redshift#ResizeClusterMessage$TargetReservedNodeOfferingId */ =>  {
                let var_21 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_target_reserved_node_offering_id(var_21);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

