// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_transit_gateway_vpc_attachment(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::TransitGatewayVpcAttachment, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::TransitGatewayVpcAttachment::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("transitGatewayAttachmentId") /* TransitGatewayAttachmentId com.amazonaws.ec2#TransitGatewayVpcAttachment$TransitGatewayAttachmentId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_transit_gateway_attachment_id(var_1);
            }
            ,
            s if s.matches("transitGatewayId") /* TransitGatewayId com.amazonaws.ec2#TransitGatewayVpcAttachment$TransitGatewayId */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_transit_gateway_id(var_2);
            }
            ,
            s if s.matches("vpcId") /* VpcId com.amazonaws.ec2#TransitGatewayVpcAttachment$VpcId */ =>  {
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
            s if s.matches("vpcOwnerId") /* VpcOwnerId com.amazonaws.ec2#TransitGatewayVpcAttachment$VpcOwnerId */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_owner_id(var_4);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#TransitGatewayVpcAttachment$State */ =>  {
                let var_5 =
                    Some(
                        Result::<crate::model::TransitGatewayAttachmentState, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::model::TransitGatewayAttachmentState::from(
                                aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_5);
            }
            ,
            s if s.matches("subnetIds") /* SubnetIds com.amazonaws.ec2#TransitGatewayVpcAttachment$SubnetIds */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_value_string_list::de_value_string_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_subnet_ids(var_6);
            }
            ,
            s if s.matches("creationTime") /* CreationTime com.amazonaws.ec2#TransitGatewayVpcAttachment$CreationTime */ =>  {
                let var_7 =
                    Some(
                        aws_smithy_types::DateTime::from_str(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , aws_smithy_types::date_time::Format::DateTime
                        )
                        .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.ec2#DateTime`)"))
                        ?
                    )
                ;
                builder = builder.set_creation_time(var_7);
            }
            ,
            s if s.matches("options") /* Options com.amazonaws.ec2#TransitGatewayVpcAttachment$Options */ =>  {
                let var_8 =
                    Some(
                        crate::protocol_serde::shape_transit_gateway_vpc_attachment_options::de_transit_gateway_vpc_attachment_options(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_options(var_8);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#TransitGatewayVpcAttachment$Tags */ =>  {
                let var_9 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_9);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

