// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_cluster_security_group(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::ClusterSecurityGroup, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::ClusterSecurityGroup::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ClusterSecurityGroupName") /* ClusterSecurityGroupName com.amazonaws.redshift#ClusterSecurityGroup$ClusterSecurityGroupName */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_cluster_security_group_name(var_1);
            }
            ,
            s if s.matches("Description") /* Description com.amazonaws.redshift#ClusterSecurityGroup$Description */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_2);
            }
            ,
            s if s.matches("EC2SecurityGroups") /* EC2SecurityGroups com.amazonaws.redshift#ClusterSecurityGroup$EC2SecurityGroups */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_ec2_security_group_list::de_ec2_security_group_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ec2_security_groups(var_3);
            }
            ,
            s if s.matches("IPRanges") /* IPRanges com.amazonaws.redshift#ClusterSecurityGroup$IPRanges */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_ip_range_list::de_ip_range_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ip_ranges(var_4);
            }
            ,
            s if s.matches("Tags") /* Tags com.amazonaws.redshift#ClusterSecurityGroup$Tags */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_5);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

