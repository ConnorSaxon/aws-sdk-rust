// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_private_ip_address_specification_list(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<std::vec::Vec<crate::model::PrivateIpAddressSpecification>, aws_smithy_xml::decode::XmlDecodeError> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("item") /* member com.amazonaws.ec2#PrivateIpAddressSpecificationList$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_private_ip_address_specification::de_private_ip_address_specification(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}

