// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_customer_gateway(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::CustomerGateway, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::CustomerGateway::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("bgpAsn") /* BgpAsn com.amazonaws.ec2#CustomerGateway$BgpAsn */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_bgp_asn(var_1);
            }
            ,
            s if s.matches("customerGatewayId") /* CustomerGatewayId com.amazonaws.ec2#CustomerGateway$CustomerGatewayId */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_customer_gateway_id(var_2);
            }
            ,
            s if s.matches("ipAddress") /* IpAddress com.amazonaws.ec2#CustomerGateway$IpAddress */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ip_address(var_3);
            }
            ,
            s if s.matches("certificateArn") /* CertificateArn com.amazonaws.ec2#CustomerGateway$CertificateArn */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_certificate_arn(var_4);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#CustomerGateway$State */ =>  {
                let var_5 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_5);
            }
            ,
            s if s.matches("type") /* Type com.amazonaws.ec2#CustomerGateway$Type */ =>  {
                let var_6 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_type(var_6);
            }
            ,
            s if s.matches("deviceName") /* DeviceName com.amazonaws.ec2#CustomerGateway$DeviceName */ =>  {
                let var_7 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_device_name(var_7);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#CustomerGateway$Tags */ =>  {
                let var_8 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_8);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

