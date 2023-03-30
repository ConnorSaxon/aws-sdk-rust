// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_subnet_ipv6_cidr_block_association_set(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<std::vec::Vec<crate::model::SubnetIpv6CidrBlockAssociation>, aws_smithy_xml::decode::XmlDecodeError> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("item") /* member com.amazonaws.ec2#SubnetIpv6CidrBlockAssociationSet$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_subnet_ipv6_cidr_block_association::de_subnet_ipv6_cidr_block_association(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}

