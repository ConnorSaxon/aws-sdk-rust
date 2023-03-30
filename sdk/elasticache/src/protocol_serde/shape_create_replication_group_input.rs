// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_replication_group_input_input(input: &crate::input::CreateReplicationGroupInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "CreateReplicationGroup", "2015-02-02");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ReplicationGroupId");
    if let Some(var_2) = &input.replication_group_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("ReplicationGroupDescription");
    if let Some(var_4) = &input.replication_group_description {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("GlobalReplicationGroupId");
    if let Some(var_6) = &input.global_replication_group_id {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("PrimaryClusterId");
    if let Some(var_8) = &input.primary_cluster_id {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("AutomaticFailoverEnabled");
    if let Some(var_10) = &input.automatic_failover_enabled {
        scope_9.boolean(*var_10);
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("MultiAZEnabled");
    if let Some(var_12) = &input.multi_az_enabled {
        scope_11.boolean(*var_12);
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("NumCacheClusters");
    if let Some(var_14) = &input.num_cache_clusters {
        scope_13.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_14).into()));
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("PreferredCacheClusterAZs");
    if let Some(var_16) = &input.preferred_cache_cluster_a_zs {
        let mut list_18 = scope_15.start_list(false, Some("AvailabilityZone"));
        for item_17 in var_16 {
            #[allow(unused_mut)]
            let mut entry_19 = list_18.entry();
            entry_19.string(item_17);
        }
        list_18.finish();
    }
    #[allow(unused_mut)]
    let mut scope_20 = writer.prefix("NumNodeGroups");
    if let Some(var_21) = &input.num_node_groups {
        scope_20.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_21).into()));
    }
    #[allow(unused_mut)]
    let mut scope_22 = writer.prefix("ReplicasPerNodeGroup");
    if let Some(var_23) = &input.replicas_per_node_group {
        scope_22.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_23).into()));
    }
    #[allow(unused_mut)]
    let mut scope_24 = writer.prefix("NodeGroupConfiguration");
    if let Some(var_25) = &input.node_group_configuration {
        let mut list_27 = scope_24.start_list(false, Some("NodeGroupConfiguration"));
        for item_26 in var_25 {
            #[allow(unused_mut)]
            let mut entry_28 = list_27.entry();
            crate::protocol_serde::shape_node_group_configuration::ser_node_group_configuration(entry_28, item_26)?;
        }
        list_27.finish();
    }
    #[allow(unused_mut)]
    let mut scope_29 = writer.prefix("CacheNodeType");
    if let Some(var_30) = &input.cache_node_type {
        scope_29.string(var_30);
    }
    #[allow(unused_mut)]
    let mut scope_31 = writer.prefix("Engine");
    if let Some(var_32) = &input.engine {
        scope_31.string(var_32);
    }
    #[allow(unused_mut)]
    let mut scope_33 = writer.prefix("EngineVersion");
    if let Some(var_34) = &input.engine_version {
        scope_33.string(var_34);
    }
    #[allow(unused_mut)]
    let mut scope_35 = writer.prefix("CacheParameterGroupName");
    if let Some(var_36) = &input.cache_parameter_group_name {
        scope_35.string(var_36);
    }
    #[allow(unused_mut)]
    let mut scope_37 = writer.prefix("CacheSubnetGroupName");
    if let Some(var_38) = &input.cache_subnet_group_name {
        scope_37.string(var_38);
    }
    #[allow(unused_mut)]
    let mut scope_39 = writer.prefix("CacheSecurityGroupNames");
    if let Some(var_40) = &input.cache_security_group_names {
        let mut list_42 = scope_39.start_list(false, Some("CacheSecurityGroupName"));
        for item_41 in var_40 {
            #[allow(unused_mut)]
            let mut entry_43 = list_42.entry();
            entry_43.string(item_41);
        }
        list_42.finish();
    }
    #[allow(unused_mut)]
    let mut scope_44 = writer.prefix("SecurityGroupIds");
    if let Some(var_45) = &input.security_group_ids {
        let mut list_47 = scope_44.start_list(false, Some("SecurityGroupId"));
        for item_46 in var_45 {
            #[allow(unused_mut)]
            let mut entry_48 = list_47.entry();
            entry_48.string(item_46);
        }
        list_47.finish();
    }
    #[allow(unused_mut)]
    let mut scope_49 = writer.prefix("Tags");
    if let Some(var_50) = &input.tags {
        let mut list_52 = scope_49.start_list(false, Some("Tag"));
        for item_51 in var_50 {
            #[allow(unused_mut)]
            let mut entry_53 = list_52.entry();
            crate::protocol_serde::shape_tag::ser_tag(entry_53, item_51)?;
        }
        list_52.finish();
    }
    #[allow(unused_mut)]
    let mut scope_54 = writer.prefix("SnapshotArns");
    if let Some(var_55) = &input.snapshot_arns {
        let mut list_57 = scope_54.start_list(false, Some("SnapshotArn"));
        for item_56 in var_55 {
            #[allow(unused_mut)]
            let mut entry_58 = list_57.entry();
            entry_58.string(item_56);
        }
        list_57.finish();
    }
    #[allow(unused_mut)]
    let mut scope_59 = writer.prefix("SnapshotName");
    if let Some(var_60) = &input.snapshot_name {
        scope_59.string(var_60);
    }
    #[allow(unused_mut)]
    let mut scope_61 = writer.prefix("PreferredMaintenanceWindow");
    if let Some(var_62) = &input.preferred_maintenance_window {
        scope_61.string(var_62);
    }
    #[allow(unused_mut)]
    let mut scope_63 = writer.prefix("Port");
    if let Some(var_64) = &input.port {
        scope_63.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_64).into()));
    }
    #[allow(unused_mut)]
    let mut scope_65 = writer.prefix("NotificationTopicArn");
    if let Some(var_66) = &input.notification_topic_arn {
        scope_65.string(var_66);
    }
    #[allow(unused_mut)]
    let mut scope_67 = writer.prefix("AutoMinorVersionUpgrade");
    if let Some(var_68) = &input.auto_minor_version_upgrade {
        scope_67.boolean(*var_68);
    }
    #[allow(unused_mut)]
    let mut scope_69 = writer.prefix("SnapshotRetentionLimit");
    if let Some(var_70) = &input.snapshot_retention_limit {
        scope_69.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_70).into()));
    }
    #[allow(unused_mut)]
    let mut scope_71 = writer.prefix("SnapshotWindow");
    if let Some(var_72) = &input.snapshot_window {
        scope_71.string(var_72);
    }
    #[allow(unused_mut)]
    let mut scope_73 = writer.prefix("AuthToken");
    if let Some(var_74) = &input.auth_token {
        scope_73.string(var_74);
    }
    #[allow(unused_mut)]
    let mut scope_75 = writer.prefix("TransitEncryptionEnabled");
    if let Some(var_76) = &input.transit_encryption_enabled {
        scope_75.boolean(*var_76);
    }
    #[allow(unused_mut)]
    let mut scope_77 = writer.prefix("AtRestEncryptionEnabled");
    if let Some(var_78) = &input.at_rest_encryption_enabled {
        scope_77.boolean(*var_78);
    }
    #[allow(unused_mut)]
    let mut scope_79 = writer.prefix("KmsKeyId");
    if let Some(var_80) = &input.kms_key_id {
        scope_79.string(var_80);
    }
    #[allow(unused_mut)]
    let mut scope_81 = writer.prefix("UserGroupIds");
    if let Some(var_82) = &input.user_group_ids {
        let mut list_84 = scope_81.start_list(false, None);
        for item_83 in var_82 {
            #[allow(unused_mut)]
            let mut entry_85 = list_84.entry();
            entry_85.string(item_83);
        }
        list_84.finish();
    }
    #[allow(unused_mut)]
    let mut scope_86 = writer.prefix("LogDeliveryConfigurations");
    if let Some(var_87) = &input.log_delivery_configurations {
        let mut list_89 = scope_86.start_list(false, Some("LogDeliveryConfigurationRequest"));
        for item_88 in var_87 {
            #[allow(unused_mut)]
            let mut entry_90 = list_89.entry();
            crate::protocol_serde::shape_log_delivery_configuration_request::ser_log_delivery_configuration_request(entry_90, item_88)?;
        }
        list_89.finish();
    }
    #[allow(unused_mut)]
    let mut scope_91 = writer.prefix("DataTieringEnabled");
    if let Some(var_92) = &input.data_tiering_enabled {
        scope_91.boolean(*var_92);
    }
    #[allow(unused_mut)]
    let mut scope_93 = writer.prefix("NetworkType");
    if let Some(var_94) = &input.network_type {
        scope_93.string(var_94.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_95 = writer.prefix("IpDiscovery");
    if let Some(var_96) = &input.ip_discovery {
        scope_95.string(var_96.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_97 = writer.prefix("TransitEncryptionMode");
    if let Some(var_98) = &input.transit_encryption_mode {
        scope_97.string(var_98.as_str());
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

