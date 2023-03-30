// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_network_acl_entry(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::NetworkAclEntry, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::NetworkAclEntry::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("cidrBlock") /* CidrBlock com.amazonaws.ec2#NetworkAclEntry$CidrBlock */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_cidr_block(var_1);
            }
            ,
            s if s.matches("egress") /* Egress com.amazonaws.ec2#NetworkAclEntry$Egress */ =>  {
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
                builder = builder.set_egress(var_2);
            }
            ,
            s if s.matches("icmpTypeCode") /* IcmpTypeCode com.amazonaws.ec2#NetworkAclEntry$IcmpTypeCode */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_icmp_type_code::de_icmp_type_code(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_icmp_type_code(var_3);
            }
            ,
            s if s.matches("ipv6CidrBlock") /* Ipv6CidrBlock com.amazonaws.ec2#NetworkAclEntry$Ipv6CidrBlock */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ipv6_cidr_block(var_4);
            }
            ,
            s if s.matches("portRange") /* PortRange com.amazonaws.ec2#NetworkAclEntry$PortRange */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_port_range::de_port_range(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_port_range(var_5);
            }
            ,
            s if s.matches("protocol") /* Protocol com.amazonaws.ec2#NetworkAclEntry$Protocol */ =>  {
                let var_6 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_protocol(var_6);
            }
            ,
            s if s.matches("ruleAction") /* RuleAction com.amazonaws.ec2#NetworkAclEntry$RuleAction */ =>  {
                let var_7 =
                    Some(
                        Result::<crate::model::RuleAction, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::RuleAction::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_rule_action(var_7);
            }
            ,
            s if s.matches("ruleNumber") /* RuleNumber com.amazonaws.ec2#NetworkAclEntry$RuleNumber */ =>  {
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
                builder = builder.set_rule_number(var_8);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

