// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_rebalance_slots_in_global_replication_group_input_input(input: &crate::input::RebalanceSlotsInGlobalReplicationGroupInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "RebalanceSlotsInGlobalReplicationGroup", "2015-02-02");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("GlobalReplicationGroupId");
    if let Some(var_2) = &input.global_replication_group_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ApplyImmediately");
     {
        scope_3.boolean(input.apply_immediately);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

