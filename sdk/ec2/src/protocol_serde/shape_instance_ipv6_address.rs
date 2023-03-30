// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_instance_ipv6_address(mut writer: aws_smithy_query::QueryValueWriter, input: &crate::model::InstanceIpv6Address) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Ipv6Address");
    if let Some(var_2) = &input.ipv6_address {
        scope_1.string(var_2);
    }
    Ok(())
}

pub fn de_instance_ipv6_address(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::InstanceIpv6Address, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::InstanceIpv6Address::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("ipv6Address") /* Ipv6Address com.amazonaws.ec2#InstanceIpv6Address$Ipv6Address */ =>  {
                let var_3 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ipv6_address(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

