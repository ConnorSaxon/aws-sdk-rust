// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_vpc_peering_connection(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::VpcPeeringConnection, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::VpcPeeringConnection::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("accepterVpcInfo") /* AccepterVpcInfo com.amazonaws.ec2#VpcPeeringConnection$AccepterVpcInfo */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_vpc_peering_connection_vpc_info::de_vpc_peering_connection_vpc_info(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_accepter_vpc_info(var_1);
            }
            ,
            s if s.matches("expirationTime") /* ExpirationTime com.amazonaws.ec2#VpcPeeringConnection$ExpirationTime */ =>  {
                let var_2 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_expiration_time(var_2);
            }
            ,
            s if s.matches("requesterVpcInfo") /* RequesterVpcInfo com.amazonaws.ec2#VpcPeeringConnection$RequesterVpcInfo */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_vpc_peering_connection_vpc_info::de_vpc_peering_connection_vpc_info(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_requester_vpc_info(var_3);
            }
            ,
            s if s.matches("status") /* Status com.amazonaws.ec2#VpcPeeringConnection$Status */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_vpc_peering_connection_state_reason::de_vpc_peering_connection_state_reason(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_status(var_4);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#VpcPeeringConnection$Tags */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_5);
            }
            ,
            s if s.matches("vpcPeeringConnectionId") /* VpcPeeringConnectionId com.amazonaws.ec2#VpcPeeringConnection$VpcPeeringConnectionId */ =>  {
                let var_6 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_peering_connection_id(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

