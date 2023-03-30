// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn de_enable_fast_snapshot_restore_error_item(decoder: &mut aws_smithy_xml::decode::ScopedDecoder) -> Result<crate::model::EnableFastSnapshotRestoreErrorItem, aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::model::EnableFastSnapshotRestoreErrorItem::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("snapshotId") /* SnapshotId com.amazonaws.ec2#EnableFastSnapshotRestoreErrorItem$SnapshotId */ =>  {
                let var_1 =
                    Some(
                        Result::<std::string::String, aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_snapshot_id(var_1);
            }
            ,
            s if s.matches("fastSnapshotRestoreStateErrorSet") /* FastSnapshotRestoreStateErrors com.amazonaws.ec2#EnableFastSnapshotRestoreErrorItem$FastSnapshotRestoreStateErrors */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_enable_fast_snapshot_restore_state_error_set::de_enable_fast_snapshot_restore_state_error_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_fast_snapshot_restore_state_errors(var_2);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build()
    )
}

