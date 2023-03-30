// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_db_instance_read_replica_input_input(input: &crate::input::CreateDbInstanceReadReplicaInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "CreateDBInstanceReadReplica", "2014-10-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DBInstanceIdentifier");
    if let Some(var_2) = &input.db_instance_identifier {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("SourceDBInstanceIdentifier");
    if let Some(var_4) = &input.source_db_instance_identifier {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("DBInstanceClass");
    if let Some(var_6) = &input.db_instance_class {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("AvailabilityZone");
    if let Some(var_8) = &input.availability_zone {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("Port");
    if let Some(var_10) = &input.port {
        scope_9.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_10).into()));
    }
    #[allow(unused_mut)]
    let mut scope_11 = writer.prefix("MultiAZ");
    if let Some(var_12) = &input.multi_az {
        scope_11.boolean(*var_12);
    }
    #[allow(unused_mut)]
    let mut scope_13 = writer.prefix("AutoMinorVersionUpgrade");
    if let Some(var_14) = &input.auto_minor_version_upgrade {
        scope_13.boolean(*var_14);
    }
    #[allow(unused_mut)]
    let mut scope_15 = writer.prefix("Iops");
    if let Some(var_16) = &input.iops {
        scope_15.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_16).into()));
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("OptionGroupName");
    if let Some(var_18) = &input.option_group_name {
        scope_17.string(var_18);
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("DBParameterGroupName");
    if let Some(var_20) = &input.db_parameter_group_name {
        scope_19.string(var_20);
    }
    #[allow(unused_mut)]
    let mut scope_21 = writer.prefix("PubliclyAccessible");
    if let Some(var_22) = &input.publicly_accessible {
        scope_21.boolean(*var_22);
    }
    #[allow(unused_mut)]
    let mut scope_23 = writer.prefix("Tags");
    if let Some(var_24) = &input.tags {
        let mut list_26 = scope_23.start_list(false, Some("Tag"));
        for item_25 in var_24 {
            #[allow(unused_mut)]
            let mut entry_27 = list_26.entry();
            crate::protocol_serde::shape_tag::ser_tag(entry_27, item_25)?;
        }
        list_26.finish();
    }
    #[allow(unused_mut)]
    let mut scope_28 = writer.prefix("DBSubnetGroupName");
    if let Some(var_29) = &input.db_subnet_group_name {
        scope_28.string(var_29);
    }
    #[allow(unused_mut)]
    let mut scope_30 = writer.prefix("VpcSecurityGroupIds");
    if let Some(var_31) = &input.vpc_security_group_ids {
        let mut list_33 = scope_30.start_list(false, Some("VpcSecurityGroupId"));
        for item_32 in var_31 {
            #[allow(unused_mut)]
            let mut entry_34 = list_33.entry();
            entry_34.string(item_32);
        }
        list_33.finish();
    }
    #[allow(unused_mut)]
    let mut scope_35 = writer.prefix("StorageType");
    if let Some(var_36) = &input.storage_type {
        scope_35.string(var_36);
    }
    #[allow(unused_mut)]
    let mut scope_37 = writer.prefix("CopyTagsToSnapshot");
    if let Some(var_38) = &input.copy_tags_to_snapshot {
        scope_37.boolean(*var_38);
    }
    #[allow(unused_mut)]
    let mut scope_39 = writer.prefix("MonitoringInterval");
    if let Some(var_40) = &input.monitoring_interval {
        scope_39.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_40).into()));
    }
    #[allow(unused_mut)]
    let mut scope_41 = writer.prefix("MonitoringRoleArn");
    if let Some(var_42) = &input.monitoring_role_arn {
        scope_41.string(var_42);
    }
    #[allow(unused_mut)]
    let mut scope_43 = writer.prefix("KmsKeyId");
    if let Some(var_44) = &input.kms_key_id {
        scope_43.string(var_44);
    }
    #[allow(unused_mut)]
    let mut scope_45 = writer.prefix("PreSignedUrl");
    if let Some(var_46) = &input.pre_signed_url {
        scope_45.string(var_46);
    }
    #[allow(unused_mut)]
    let mut scope_47 = writer.prefix("EnableIAMDatabaseAuthentication");
    if let Some(var_48) = &input.enable_iam_database_authentication {
        scope_47.boolean(*var_48);
    }
    #[allow(unused_mut)]
    let mut scope_49 = writer.prefix("EnablePerformanceInsights");
    if let Some(var_50) = &input.enable_performance_insights {
        scope_49.boolean(*var_50);
    }
    #[allow(unused_mut)]
    let mut scope_51 = writer.prefix("PerformanceInsightsKMSKeyId");
    if let Some(var_52) = &input.performance_insights_kms_key_id {
        scope_51.string(var_52);
    }
    #[allow(unused_mut)]
    let mut scope_53 = writer.prefix("PerformanceInsightsRetentionPeriod");
    if let Some(var_54) = &input.performance_insights_retention_period {
        scope_53.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_54).into()));
    }
    #[allow(unused_mut)]
    let mut scope_55 = writer.prefix("EnableCloudwatchLogsExports");
    if let Some(var_56) = &input.enable_cloudwatch_logs_exports {
        let mut list_58 = scope_55.start_list(false, None);
        for item_57 in var_56 {
            #[allow(unused_mut)]
            let mut entry_59 = list_58.entry();
            entry_59.string(item_57);
        }
        list_58.finish();
    }
    #[allow(unused_mut)]
    let mut scope_60 = writer.prefix("ProcessorFeatures");
    if let Some(var_61) = &input.processor_features {
        let mut list_63 = scope_60.start_list(false, Some("ProcessorFeature"));
        for item_62 in var_61 {
            #[allow(unused_mut)]
            let mut entry_64 = list_63.entry();
            crate::protocol_serde::shape_processor_feature::ser_processor_feature(entry_64, item_62)?;
        }
        list_63.finish();
    }
    #[allow(unused_mut)]
    let mut scope_65 = writer.prefix("UseDefaultProcessorFeatures");
    if let Some(var_66) = &input.use_default_processor_features {
        scope_65.boolean(*var_66);
    }
    #[allow(unused_mut)]
    let mut scope_67 = writer.prefix("DeletionProtection");
    if let Some(var_68) = &input.deletion_protection {
        scope_67.boolean(*var_68);
    }
    #[allow(unused_mut)]
    let mut scope_69 = writer.prefix("Domain");
    if let Some(var_70) = &input.domain {
        scope_69.string(var_70);
    }
    #[allow(unused_mut)]
    let mut scope_71 = writer.prefix("DomainIAMRoleName");
    if let Some(var_72) = &input.domain_iam_role_name {
        scope_71.string(var_72);
    }
    #[allow(unused_mut)]
    let mut scope_73 = writer.prefix("ReplicaMode");
    if let Some(var_74) = &input.replica_mode {
        scope_73.string(var_74.as_str());
    }
    #[allow(unused_mut)]
    let mut scope_75 = writer.prefix("MaxAllocatedStorage");
    if let Some(var_76) = &input.max_allocated_storage {
        scope_75.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_76).into()));
    }
    #[allow(unused_mut)]
    let mut scope_77 = writer.prefix("CustomIamInstanceProfile");
    if let Some(var_78) = &input.custom_iam_instance_profile {
        scope_77.string(var_78);
    }
    #[allow(unused_mut)]
    let mut scope_79 = writer.prefix("NetworkType");
    if let Some(var_80) = &input.network_type {
        scope_79.string(var_80);
    }
    #[allow(unused_mut)]
    let mut scope_81 = writer.prefix("StorageThroughput");
    if let Some(var_82) = &input.storage_throughput {
        scope_81.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_82).into()));
    }
    #[allow(unused_mut)]
    let mut scope_83 = writer.prefix("EnableCustomerOwnedIp");
    if let Some(var_84) = &input.enable_customer_owned_ip {
        scope_83.boolean(*var_84);
    }
    #[allow(unused_mut)]
    let mut scope_85 = writer.prefix("AllocatedStorage");
    if let Some(var_86) = &input.allocated_storage {
        scope_85.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_86).into()));
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

