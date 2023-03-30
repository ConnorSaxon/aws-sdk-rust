// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_load_balancer(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::LoadBalancer, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::LoadBalancer::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("LoadBalancerArn") /* LoadBalancerArn com.amazonaws.elasticloadbalancingv2#LoadBalancer$LoadBalancerArn */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_load_balancer_arn(var_1);
            }
            ,
            s if s.matches("DNSName") /* DNSName com.amazonaws.elasticloadbalancingv2#LoadBalancer$DNSName */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_dns_name(var_2);
            }
            ,
            s if s.matches("CanonicalHostedZoneId") /* CanonicalHostedZoneId com.amazonaws.elasticloadbalancingv2#LoadBalancer$CanonicalHostedZoneId */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_canonical_hosted_zone_id(var_3);
            }
            ,
            s if s.matches("CreatedTime") /* CreatedTime com.amazonaws.elasticloadbalancingv2#LoadBalancer$CreatedTime */ =>  {
                let var_4 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.elasticloadbalancingv2#CreatedTime`)"))
                        ?
                    )
                ;
                builder = builder.set_created_time(var_4);
            }
            ,
            s if s.matches("LoadBalancerName") /* LoadBalancerName com.amazonaws.elasticloadbalancingv2#LoadBalancer$LoadBalancerName */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_load_balancer_name(var_5);
            }
            ,
            s if s.matches("Scheme") /* Scheme com.amazonaws.elasticloadbalancingv2#LoadBalancer$Scheme */ =>  {
                let var_6 =
                    Some(
                        Result::<crate::model::LoadBalancerSchemeEnum, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::LoadBalancerSchemeEnum::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_scheme(var_6);
            }
            ,
            s if s.matches("VpcId") /* VpcId com.amazonaws.elasticloadbalancingv2#LoadBalancer$VpcId */ =>  {
                let var_7 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_id(var_7);
            }
            ,
            s if s.matches("State") /* State com.amazonaws.elasticloadbalancingv2#LoadBalancer$State */ =>  {
                let var_8 =
                    Some(
                        crate::protocol_serde::shape_load_balancer_state::de_load_balancer_state(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_state(var_8);
            }
            ,
            s if s.matches("Type") /* Type com.amazonaws.elasticloadbalancingv2#LoadBalancer$Type */ =>  {
                let var_9 =
                    Some(
                        Result::<crate::model::LoadBalancerTypeEnum, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::LoadBalancerTypeEnum::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_type(var_9);
            }
            ,
            s if s.matches("AvailabilityZones") /* AvailabilityZones com.amazonaws.elasticloadbalancingv2#LoadBalancer$AvailabilityZones */ =>  {
                let var_10 =
                    Some(
                        crate::protocol_serde::shape_availability_zones::de_availability_zones(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_availability_zones(var_10);
            }
            ,
            s if s.matches("SecurityGroups") /* SecurityGroups com.amazonaws.elasticloadbalancingv2#LoadBalancer$SecurityGroups */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_security_groups::de_security_groups(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_security_groups(var_11);
            }
            ,
            s if s.matches("IpAddressType") /* IpAddressType com.amazonaws.elasticloadbalancingv2#LoadBalancer$IpAddressType */ =>  {
                let var_12 =
                    Some(
                        Result::<crate::model::IpAddressType, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::IpAddressType::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_ip_address_type(var_12);
            }
            ,
            s if s.matches("CustomerOwnedIpv4Pool") /* CustomerOwnedIpv4Pool com.amazonaws.elasticloadbalancingv2#LoadBalancer$CustomerOwnedIpv4Pool */ =>  {
                let var_13 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_customer_owned_ipv4_pool(var_13);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

