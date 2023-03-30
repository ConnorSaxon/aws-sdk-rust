// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_db_security_group_membership_list(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<std::vec::Vec<crate::model::DbSecurityGroupMembership>, aws_smithy_xml::decode::XmlDecodeError> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("DBSecurityGroup") /* member com.amazonaws.rds#DBSecurityGroupMembershipList$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_db_security_group_membership::de_db_security_group_membership(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}

