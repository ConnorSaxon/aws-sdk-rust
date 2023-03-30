// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_copy_snapshot_input_input(input: &crate::input::CopySnapshotInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "CopySnapshot", "2015-02-02");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("SourceSnapshotName");
    if let Some(var_2) = &input.source_snapshot_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("TargetSnapshotName");
    if let Some(var_4) = &input.target_snapshot_name {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("TargetBucket");
    if let Some(var_6) = &input.target_bucket {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("KmsKeyId");
    if let Some(var_8) = &input.kms_key_id {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("Tags");
    if let Some(var_10) = &input.tags {
        let mut list_12 = scope_9.start_list(false, Some("Tag"));
        for item_11 in var_10 {
            #[allow(unused_mut)]
            let mut entry_13 = list_12.entry();
            crate::protocol_serde::shape_tag::ser_tag(entry_13, item_11)?;
        }
        list_12.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

