// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_replication_group_input_input(input: &crate::input::DeleteReplicationGroupInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "DeleteReplicationGroup", "2015-02-02");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ReplicationGroupId");
    if let Some(var_2) = &input.replication_group_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("RetainPrimaryCluster");
    if let Some(var_4) = &input.retain_primary_cluster {
        scope_3.boolean(*var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("FinalSnapshotIdentifier");
    if let Some(var_6) = &input.final_snapshot_identifier {
        scope_5.string(var_6);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

