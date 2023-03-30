// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_traffic_mirror_target(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::TrafficMirrorTarget, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::TrafficMirrorTarget::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("trafficMirrorTargetId") /* TrafficMirrorTargetId com.amazonaws.ec2#TrafficMirrorTarget$TrafficMirrorTargetId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_traffic_mirror_target_id(var_1);
            }
            ,
            s if s.matches("networkInterfaceId") /* NetworkInterfaceId com.amazonaws.ec2#TrafficMirrorTarget$NetworkInterfaceId */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_network_interface_id(var_2);
            }
            ,
            s if s.matches("networkLoadBalancerArn") /* NetworkLoadBalancerArn com.amazonaws.ec2#TrafficMirrorTarget$NetworkLoadBalancerArn */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_network_load_balancer_arn(var_3);
            }
            ,
            s if s.matches("type") /* Type com.amazonaws.ec2#TrafficMirrorTarget$Type */ =>  {
                let var_4 =
                    Some(
                        Result::<crate::model::TrafficMirrorTargetType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::TrafficMirrorTargetType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_type(var_4);
            }
            ,
            s if s.matches("description") /* Description com.amazonaws.ec2#TrafficMirrorTarget$Description */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_5);
            }
            ,
            s if s.matches("ownerId") /* OwnerId com.amazonaws.ec2#TrafficMirrorTarget$OwnerId */ =>  {
                let var_6 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_owner_id(var_6);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#TrafficMirrorTarget$Tags */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_7);
            }
            ,
            s if s.matches("gatewayLoadBalancerEndpointId") /* GatewayLoadBalancerEndpointId com.amazonaws.ec2#TrafficMirrorTarget$GatewayLoadBalancerEndpointId */ =>  {
                let var_8 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_gateway_load_balancer_endpoint_id(var_8);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

