// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_db_subnet_group(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::DbSubnetGroup, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::DbSubnetGroup::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("DBSubnetGroupName") /* DBSubnetGroupName com.amazonaws.neptune#DBSubnetGroup$DBSubnetGroupName */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_subnet_group_name(var_1);
            }
            ,
            s if s.matches("DBSubnetGroupDescription") /* DBSubnetGroupDescription com.amazonaws.neptune#DBSubnetGroup$DBSubnetGroupDescription */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_subnet_group_description(var_2);
            }
            ,
            s if s.matches("VpcId") /* VpcId com.amazonaws.neptune#DBSubnetGroup$VpcId */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_id(var_3);
            }
            ,
            s if s.matches("SubnetGroupStatus") /* SubnetGroupStatus com.amazonaws.neptune#DBSubnetGroup$SubnetGroupStatus */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_subnet_group_status(var_4);
            }
            ,
            s if s.matches("Subnets") /* Subnets com.amazonaws.neptune#DBSubnetGroup$Subnets */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_subnet_list::de_subnet_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_subnets(var_5);
            }
            ,
            s if s.matches("DBSubnetGroupArn") /* DBSubnetGroupArn com.amazonaws.neptune#DBSubnetGroup$DBSubnetGroupArn */ =>  {
                let var_6 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_db_subnet_group_arn(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

