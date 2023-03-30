// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_db_cluster_snapshot_attribute_list(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<std::vec::Vec<crate::model::DbClusterSnapshotAttribute>, aws_smithy_xml::decode::XmlDecodeError> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("DBClusterSnapshotAttribute") /* member com.amazonaws.neptune#DBClusterSnapshotAttributeList$member */ =>  {
                out.push(
                    crate::protocol_serde::shape_db_cluster_snapshot_attribute::de_db_cluster_snapshot_attribute(&mut tag)
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}

