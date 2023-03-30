// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_analysis_route_table_route(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::AnalysisRouteTableRoute, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::AnalysisRouteTableRoute::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("destinationCidr") /* DestinationCidr com.amazonaws.ec2#AnalysisRouteTableRoute$DestinationCidr */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_destination_cidr(var_1);
            }
            ,
            s if s.matches("destinationPrefixListId") /* DestinationPrefixListId com.amazonaws.ec2#AnalysisRouteTableRoute$DestinationPrefixListId */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_destination_prefix_list_id(var_2);
            }
            ,
            s if s.matches("egressOnlyInternetGatewayId") /* EgressOnlyInternetGatewayId com.amazonaws.ec2#AnalysisRouteTableRoute$EgressOnlyInternetGatewayId */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_egress_only_internet_gateway_id(var_3);
            }
            ,
            s if s.matches("gatewayId") /* GatewayId com.amazonaws.ec2#AnalysisRouteTableRoute$GatewayId */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_gateway_id(var_4);
            }
            ,
            s if s.matches("instanceId") /* InstanceId com.amazonaws.ec2#AnalysisRouteTableRoute$InstanceId */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_id(var_5);
            }
            ,
            s if s.matches("natGatewayId") /* NatGatewayId com.amazonaws.ec2#AnalysisRouteTableRoute$NatGatewayId */ =>  {
                let var_6 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_nat_gateway_id(var_6);
            }
            ,
            s if s.matches("networkInterfaceId") /* NetworkInterfaceId com.amazonaws.ec2#AnalysisRouteTableRoute$NetworkInterfaceId */ =>  {
                let var_7 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_network_interface_id(var_7);
            }
            ,
            s if s.matches("origin") /* Origin com.amazonaws.ec2#AnalysisRouteTableRoute$Origin */ =>  {
                let var_8 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_origin(var_8);
            }
            ,
            s if s.matches("transitGatewayId") /* TransitGatewayId com.amazonaws.ec2#AnalysisRouteTableRoute$TransitGatewayId */ =>  {
                let var_9 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_transit_gateway_id(var_9);
            }
            ,
            s if s.matches("vpcPeeringConnectionId") /* VpcPeeringConnectionId com.amazonaws.ec2#AnalysisRouteTableRoute$VpcPeeringConnectionId */ =>  {
                let var_10 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_peering_connection_id(var_10);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#AnalysisRouteTableRoute$State */ =>  {
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
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

