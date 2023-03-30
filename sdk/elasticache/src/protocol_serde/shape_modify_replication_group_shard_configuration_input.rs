// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_replication_group_shard_configuration_input_input(input: &crate::input::ModifyReplicationGroupShardConfigurationInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "ModifyReplicationGroupShardConfiguration", "2015-02-02");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ReplicationGroupId");
    if let Some(var_2) = &input.replication_group_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("NodeGroupCount");
     {
        scope_3.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.node_group_count).into()));
    }
    #[allow(unused_mut)]
    let mut scope_4 = writer.prefix("ApplyImmediately");
     {
        scope_4.boolean(input.apply_immediately);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("ReshardingConfiguration");
    if let Some(var_6) = &input.resharding_configuration {
        let mut list_8 = scope_5.start_list(false, Some("ReshardingConfiguration"));
        for item_7 in var_6 {
            #[allow(unused_mut)]
            let mut entry_9 = list_8.entry();
            crate::protocol_serde::shape_resharding_configuration::ser_resharding_configuration(entry_9, item_7)?;
        }
        list_8.finish();
    }
    #[allow(unused_mut)]
    let mut scope_10 = writer.prefix("NodeGroupsToRemove");
    if let Some(var_11) = &input.node_groups_to_remove {
        let mut list_13 = scope_10.start_list(false, Some("NodeGroupToRemove"));
        for item_12 in var_11 {
            #[allow(unused_mut)]
            let mut entry_14 = list_13.entry();
            entry_14.string(item_12);
        }
        list_13.finish();
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("NodeGroupsToRetain");
    if let Some(var_16) = &input.node_groups_to_retain {
        let mut list_18 = scope_15.start_list(false, Some("NodeGroupToRetain"));
        for item_17 in var_16 {
            #[allow(unused_mut)]
            let mut entry_19 = list_18.entry();
            entry_19.string(item_17);
        }
        list_18.finish();
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

