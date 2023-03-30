// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_launch_template_placement(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::LaunchTemplatePlacement, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::LaunchTemplatePlacement::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("availabilityZone") /* AvailabilityZone com.amazonaws.ec2#LaunchTemplatePlacement$AvailabilityZone */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_availability_zone(var_1);
            }
            ,
            s if s.matches("affinity") /* Affinity com.amazonaws.ec2#LaunchTemplatePlacement$Affinity */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_affinity(var_2);
            }
            ,
            s if s.matches("groupName") /* GroupName com.amazonaws.ec2#LaunchTemplatePlacement$GroupName */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_group_name(var_3);
            }
            ,
            s if s.matches("hostId") /* HostId com.amazonaws.ec2#LaunchTemplatePlacement$HostId */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_host_id(var_4);
            }
            ,
            s if s.matches("tenancy") /* Tenancy com.amazonaws.ec2#LaunchTemplatePlacement$Tenancy */ =>  {
                let var_5 =
                    Some(
                        Result::<crate::model::Tenancy, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::Tenancy::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_tenancy(var_5);
            }
            ,
            s if s.matches("spreadDomain") /* SpreadDomain com.amazonaws.ec2#LaunchTemplatePlacement$SpreadDomain */ =>  {
                let var_6 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_spread_domain(var_6);
            }
            ,
            s if s.matches("hostResourceGroupArn") /* HostResourceGroupArn com.amazonaws.ec2#LaunchTemplatePlacement$HostResourceGroupArn */ =>  {
                let var_7 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_host_resource_group_arn(var_7);
            }
            ,
            s if s.matches("partitionNumber") /* PartitionNumber com.amazonaws.ec2#LaunchTemplatePlacement$PartitionNumber */ =>  {
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
                builder = builder.set_partition_number(var_8);
            }
            ,
            s if s.matches("groupId") /* GroupId com.amazonaws.ec2#LaunchTemplatePlacement$GroupId */ =>  {
                let var_9 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_group_id(var_9);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

