// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_launch_template_instance_network_interface_specification(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::LaunchTemplateInstanceNetworkInterfaceSpecification, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::LaunchTemplateInstanceNetworkInterfaceSpecification::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("associateCarrierIpAddress") /* AssociateCarrierIpAddress com.amazonaws.ec2#LaunchTemplateInstanceNetworkInterfaceSpecification$AssociateCarrierIpAddress */ =>  {
                let var_1 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_associate_carrier_ip_address(var_1);
            }
            ,
            s if s.matches("associatePublicIpAddress") /* AssociatePublicIpAddress com.amazonaws.ec2#LaunchTemplateInstanceNetworkInterfaceSpecification$AssociatePublicIpAddress */ =>  {
                let var_2 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_associate_public_ip_address(var_2);
            }
            ,
            s if s.matches("deleteOnTermination") /* DeleteOnTermination com.amazonaws.ec2#LaunchTemplateInstanceNetworkInterfaceSpecification$DeleteOnTermination */ =>  {
                let var_3 =
                    Some(
                         {
                            <bool as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_delete_on_termination(var_3);
            }
            ,
            s if s.matches("description") /* Description com.amazonaws.ec2#LaunchTemplateInstanceNetworkInterfaceSpecification$Description */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_4);
            }
            ,
            s if s.matches("deviceIndex") /* DeviceIndex com.amazonaws.ec2#LaunchTemplateInstanceNetworkInterfaceSpecification$DeviceIndex */ =>  {
                let var_5 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_device_index(var_5);
            }
            ,
            s if s.matches("groupSet") /* Groups com.amazonaws.ec2#LaunchTemplateInstanceNetworkInterfaceSpecification$Groups */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_group_id_string_list::de_group_id_string_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_groups(var_6);
            }
            ,
            s if s.matches("interfaceType") /* InterfaceType com.amazonaws.ec2#LaunchTemplateInstanceNetworkInterfaceSpecification$InterfaceType */ =>  {
                let var_7 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_interface_type(var_7);
            }
            ,
            s if s.matches("ipv6AddressCount") /* Ipv6AddressCount com.amazonaws.ec2#LaunchTemplateInstanceNetworkInterfaceSpecification$Ipv6AddressCount */ =>  {
                let var_8 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_ipv6_address_count(var_8);
            }
            ,
            s if s.matches("ipv6AddressesSet") /* Ipv6Addresses com.amazonaws.ec2#LaunchTemplateInstanceNetworkInterfaceSpecification$Ipv6Addresses */ =>  {
                let var_9 =
                    Some(
                        crate::protocol_serde::shape_instance_ipv6_address_list::de_instance_ipv6_address_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ipv6_addresses(var_9);
            }
            ,
            s if s.matches("networkInterfaceId") /* NetworkInterfaceId com.amazonaws.ec2#LaunchTemplateInstanceNetworkInterfaceSpecification$NetworkInterfaceId */ =>  {
                let var_10 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_network_interface_id(var_10);
            }
            ,
            s if s.matches("privateIpAddress") /* PrivateIpAddress com.amazonaws.ec2#LaunchTemplateInstanceNetworkInterfaceSpecification$PrivateIpAddress */ =>  {
                let var_11 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_private_ip_address(var_11);
            }
            ,
            s if s.matches("privateIpAddressesSet") /* PrivateIpAddresses com.amazonaws.ec2#LaunchTemplateInstanceNetworkInterfaceSpecification$PrivateIpAddresses */ =>  {
                let var_12 =
                    Some(
                        crate::protocol_serde::shape_private_ip_address_specification_list::de_private_ip_address_specification_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_private_ip_addresses(var_12);
            }
            ,
            s if s.matches("secondaryPrivateIpAddressCount") /* SecondaryPrivateIpAddressCount com.amazonaws.ec2#LaunchTemplateInstanceNetworkInterfaceSpecification$SecondaryPrivateIpAddressCount */ =>  {
                let var_13 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_secondary_private_ip_address_count(var_13);
            }
            ,
            s if s.matches("subnetId") /* SubnetId com.amazonaws.ec2#LaunchTemplateInstanceNetworkInterfaceSpecification$SubnetId */ =>  {
                let var_14 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_subnet_id(var_14);
            }
            ,
            s if s.matches("networkCardIndex") /* NetworkCardIndex com.amazonaws.ec2#LaunchTemplateInstanceNetworkInterfaceSpecification$NetworkCardIndex */ =>  {
                let var_15 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_network_card_index(var_15);
            }
            ,
            s if s.matches("ipv4PrefixSet") /* Ipv4Prefixes com.amazonaws.ec2#LaunchTemplateInstanceNetworkInterfaceSpecification$Ipv4Prefixes */ =>  {
                let var_16 =
                    Some(
                        crate::protocol_serde::shape_ipv4_prefix_list_response::de_ipv4_prefix_list_response(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ipv4_prefixes(var_16);
            }
            ,
            s if s.matches("ipv4PrefixCount") /* Ipv4PrefixCount com.amazonaws.ec2#LaunchTemplateInstanceNetworkInterfaceSpecification$Ipv4PrefixCount */ =>  {
                let var_17 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_ipv4_prefix_count(var_17);
            }
            ,
            s if s.matches("ipv6PrefixSet") /* Ipv6Prefixes com.amazonaws.ec2#LaunchTemplateInstanceNetworkInterfaceSpecification$Ipv6Prefixes */ =>  {
                let var_18 =
                    Some(
                        crate::protocol_serde::shape_ipv6_prefix_list_response::de_ipv6_prefix_list_response(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ipv6_prefixes(var_18);
            }
            ,
            s if s.matches("ipv6PrefixCount") /* Ipv6PrefixCount com.amazonaws.ec2#LaunchTemplateInstanceNetworkInterfaceSpecification$Ipv6PrefixCount */ =>  {
                let var_19 =
                    Some(
                         {
                            <i32 as aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_ipv6_prefix_count(var_19);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

