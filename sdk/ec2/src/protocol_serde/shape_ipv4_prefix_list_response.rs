// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_ipv4_prefix_list_response(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<std::vec::Vec<crate::model::Ipv4PrefixSpecificationResponse>, aws_smithy_xml::decode::XmlDecodeError> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("item") /* member com.amazonaws.ec2#Ipv4PrefixListResponse$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_ipv4_prefix_specification_response::de_ipv4_prefix_specification_response(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}

