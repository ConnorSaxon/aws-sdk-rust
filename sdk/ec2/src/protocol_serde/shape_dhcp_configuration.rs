// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_dhcp_configuration(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::DhcpConfiguration, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::DhcpConfiguration::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("key") /* Key com.amazonaws.ec2#DhcpConfiguration$Key */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_key(var_1);
            }
            ,
            s if s.matches("valueSet") /* Values com.amazonaws.ec2#DhcpConfiguration$Values */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_dhcp_configuration_value_list::de_dhcp_configuration_value_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_values(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

