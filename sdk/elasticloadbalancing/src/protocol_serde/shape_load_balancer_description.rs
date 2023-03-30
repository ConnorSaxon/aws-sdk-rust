// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_load_balancer_description(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::LoadBalancerDescription, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::LoadBalancerDescription::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("LoadBalancerName") /* LoadBalancerName com.amazonaws.elasticloadbalancing#LoadBalancerDescription$LoadBalancerName */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_load_balancer_name(var_1);
            }
            ,
            s if s.matches("DNSName") /* DNSName com.amazonaws.elasticloadbalancing#LoadBalancerDescription$DNSName */ =>  {
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
            s if s.matches("CanonicalHostedZoneName") /* CanonicalHostedZoneName com.amazonaws.elasticloadbalancing#LoadBalancerDescription$CanonicalHostedZoneName */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_canonical_hosted_zone_name(var_3);
            }
            ,
            s if s.matches("CanonicalHostedZoneNameID") /* CanonicalHostedZoneNameID com.amazonaws.elasticloadbalancing#LoadBalancerDescription$CanonicalHostedZoneNameID */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_canonical_hosted_zone_name_id(var_4);
            }
            ,
            s if s.matches("ListenerDescriptions") /* ListenerDescriptions com.amazonaws.elasticloadbalancing#LoadBalancerDescription$ListenerDescriptions */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_listener_descriptions::de_listener_descriptions(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_listener_descriptions(var_5);
            }
            ,
            s if s.matches("Policies") /* Policies com.amazonaws.elasticloadbalancing#LoadBalancerDescription$Policies */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_policies::de_policies(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_policies(var_6);
            }
            ,
            s if s.matches("BackendServerDescriptions") /* BackendServerDescriptions com.amazonaws.elasticloadbalancing#LoadBalancerDescription$BackendServerDescriptions */ =>  {
                let var_7 =
                    Some(
                        crate::protocol_serde::shape_backend_server_descriptions::de_backend_server_descriptions(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_backend_server_descriptions(var_7);
            }
            ,
            s if s.matches("AvailabilityZones") /* AvailabilityZones com.amazonaws.elasticloadbalancing#LoadBalancerDescription$AvailabilityZones */ =>  {
                let var_8 =
                    Some(
                        crate::protocol_serde::shape_availability_zones::de_availability_zones(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_availability_zones(var_8);
            }
            ,
            s if s.matches("Subnets") /* Subnets com.amazonaws.elasticloadbalancing#LoadBalancerDescription$Subnets */ =>  {
                let var_9 =
                    Some(
                        crate::protocol_serde::shape_subnets::de_subnets(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_subnets(var_9);
            }
            ,
            s if s.matches("VPCId") /* VPCId com.amazonaws.elasticloadbalancing#LoadBalancerDescription$VPCId */ =>  {
                let var_10 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_id(var_10);
            }
            ,
            s if s.matches("Instances") /* Instances com.amazonaws.elasticloadbalancing#LoadBalancerDescription$Instances */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_instances::de_instances(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_instances(var_11);
            }
            ,
            s if s.matches("HealthCheck") /* HealthCheck com.amazonaws.elasticloadbalancing#LoadBalancerDescription$HealthCheck */ =>  {
                let var_12 =
                    Some(
                        crate::protocol_serde::shape_health_check::de_health_check(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_health_check(var_12);
            }
            ,
            s if s.matches("SourceSecurityGroup") /* SourceSecurityGroup com.amazonaws.elasticloadbalancing#LoadBalancerDescription$SourceSecurityGroup */ =>  {
                let var_13 =
                    Some(
                        crate::protocol_serde::shape_source_security_group::de_source_security_group(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_source_security_group(var_13);
            }
            ,
            s if s.matches("SecurityGroups") /* SecurityGroups com.amazonaws.elasticloadbalancing#LoadBalancerDescription$SecurityGroups */ =>  {
                let var_14 =
                    Some(
                        crate::protocol_serde::shape_security_groups::de_security_groups(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_security_groups(var_14);
            }
            ,
            s if s.matches("CreatedTime") /* CreatedTime com.amazonaws.elasticloadbalancing#LoadBalancerDescription$CreatedTime */ =>  {
                let var_15 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.elasticloadbalancing#CreatedTime`)"))
                        ?
                    )
                ;
                builder = builder.set_created_time(var_15);
            }
            ,
            s if s.matches("Scheme") /* Scheme com.amazonaws.elasticloadbalancing#LoadBalancerDescription$Scheme */ =>  {
                let var_16 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_scheme(var_16);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

