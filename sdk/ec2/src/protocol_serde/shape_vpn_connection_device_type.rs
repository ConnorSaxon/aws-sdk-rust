// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_vpn_connection_device_type(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::VpnConnectionDeviceType, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::VpnConnectionDeviceType::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("vpnConnectionDeviceTypeId") /* VpnConnectionDeviceTypeId com.amazonaws.ec2#VpnConnectionDeviceType$VpnConnectionDeviceTypeId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpn_connection_device_type_id(var_1);
            }
            ,
            s if s.matches("vendor") /* Vendor com.amazonaws.ec2#VpnConnectionDeviceType$Vendor */ =>  {
                let var_2 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vendor(var_2);
            }
            ,
            s if s.matches("platform") /* Platform com.amazonaws.ec2#VpnConnectionDeviceType$Platform */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_platform(var_3);
            }
            ,
            s if s.matches("software") /* Software com.amazonaws.ec2#VpnConnectionDeviceType$Software */ =>  {
                let var_4 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_software(var_4);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

