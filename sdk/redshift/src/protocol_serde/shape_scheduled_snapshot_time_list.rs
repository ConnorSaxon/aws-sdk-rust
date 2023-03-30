// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_scheduled_snapshot_time_list(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<std::vec::Vec<aws_smithy_types::DateTime>, aws_smithy_xml::decode::XmlDecodeError> {
    let mut out = std::vec::Vec::new();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("SnapshotTime") /* member com.amazonaws.redshift#ScheduledSnapshotTimeList$member */ =>  {
                out.push(
                    aws_smithy_types::DateTime::from_str(
                        aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                        , aws_smithy_types::date_time::Format::DateTime
                    )
                    .map_err(|_|aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.redshift#TStamp`)"))
                    ?
                );
            }
            ,
            _ => {}
        }
    }
    Ok(out)
}

