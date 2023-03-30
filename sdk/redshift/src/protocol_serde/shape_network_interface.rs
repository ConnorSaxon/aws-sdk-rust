// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_network_interface(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::NetworkInterface, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::NetworkInterface::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("NetworkInterfaceId") /* NetworkInterfaceId com.amazonaws.redshift#NetworkInterface$NetworkInterfaceId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_network_interface_id(var_1);
            }
            ,
            s if s.matches("SubnetId") /* SubnetId com.amazonaws.redshift#NetworkInterface$SubnetId */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_subnet_id(var_2);
            }
            ,
            s if s.matches("PrivateIpAddress") /* PrivateIpAddress com.amazonaws.redshift#NetworkInterface$PrivateIpAddress */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_private_ip_address(var_3);
            }
            ,
            s if s.matches("AvailabilityZone") /* AvailabilityZone com.amazonaws.redshift#NetworkInterface$AvailabilityZone */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_availability_zone(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

