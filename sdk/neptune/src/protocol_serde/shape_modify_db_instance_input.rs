// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_db_instance_input_input(input: &crate::input::ModifyDbInstanceInput) -> Result<aws_smithy_http::body::SdkBody, aws_smithy_http::operation::error::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = aws_smithy_query::QueryWriter::new(&mut out, "ModifyDBInstance", "2014-10-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DBInstanceIdentifier");
    if let Some(var_2) = &input.db_instance_identifier {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("AllocatedStorage");
    if let Some(var_4) = &input.allocated_storage {
        scope_3.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_4).into()));
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("DBInstanceClass");
    if let Some(var_6) = &input.db_instance_class {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("DBSubnetGroupName");
    if let Some(var_8) = &input.db_subnet_group_name {
        scope_7.string(var_8);
    }
    #[allow(unused_mut)]
    let mut scope_9 = writer.prefix("DBSecurityGroups");
    if let Some(var_10) = &input.db_security_groups {
        let mut list_12 = scope_9.start_list(false, Some("DBSecurityGroupName"));
        for item_11 in var_10 {
            #[allow(unused_mut)]
            let mut entry_13 = list_12.entry();
            entry_13.string(item_11);
        }
        list_12.finish();
    }
    #[allow(unused_mut)]
    let mut scope_14 = writer.prefix("VpcSecurityGroupIds");
    if let Some(var_15) = &input.vpc_security_group_ids {
        let mut list_17 = scope_14.start_list(false, Some("VpcSecurityGroupId"));
        for item_16 in var_15 {
            #[allow(unused_mut)]
            let mut entry_18 = list_17.entry();
            entry_18.string(item_16);
        }
        list_17.finish();
    }
    #[allow(unused_mut)]
    let mut scope_19 = writer.prefix("ApplyImmediately");
    if input.apply_immediately {
        scope_19.boolean(input.apply_immediately);
    }
    #[allow(unused_mut)]
    let mut scope_20 = writer.prefix("MasterUserPassword");
    if let Some(var_21) = &input.master_user_password {
        scope_20.string(var_21);
    }
    #[allow(unused_mut)]
    let mut scope_22 = writer.prefix("DBParameterGroupName");
    if let Some(var_23) = &input.db_parameter_group_name {
        scope_22.string(var_23);
    }
    #[allow(unused_mut)]
    let mut scope_24 = writer.prefix("BackupRetentionPeriod");
    if let Some(var_25) = &input.backup_retention_period {
        scope_24.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_25).into()));
    }
    #[allow(unused_mut)]
    let mut scope_26 = writer.prefix("PreferredBackupWindow");
    if let Some(var_27) = &input.preferred_backup_window {
        scope_26.string(var_27);
    }
    #[allow(unused_mut)]
    let mut scope_28 = writer.prefix("PreferredMaintenanceWindow");
    if let Some(var_29) = &input.preferred_maintenance_window {
        scope_28.string(var_29);
    }
    #[allow(unused_mut)]
    let mut scope_30 = writer.prefix("MultiAZ");
    if let Some(var_31) = &input.multi_az {
        scope_30.boolean(*var_31);
    }
    #[allow(unused_mut)]
    let mut scope_32 = writer.prefix("EngineVersion");
    if let Some(var_33) = &input.engine_version {
        scope_32.string(var_33);
    }
    #[allow(unused_mut)]
    let mut scope_34 = writer.prefix("AllowMajorVersionUpgrade");
    if input.allow_major_version_upgrade {
        scope_34.boolean(input.allow_major_version_upgrade);
    }
    #[allow(unused_mut)]
    let mut scope_35 = writer.prefix("AutoMinorVersionUpgrade");
    if let Some(var_36) = &input.auto_minor_version_upgrade {
        scope_35.boolean(*var_36);
    }
    #[allow(unused_mut)]
    let mut scope_37 = writer.prefix("LicenseModel");
    if let Some(var_38) = &input.license_model {
        scope_37.string(var_38);
    }
    #[allow(unused_mut)]
    let mut scope_39 = writer.prefix("Iops");
    if let Some(var_40) = &input.iops {
        scope_39.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_40).into()));
    }
    #[allow(unused_mut)]
    let mut scope_41 = writer.prefix("OptionGroupName");
    if let Some(var_42) = &input.option_group_name {
        scope_41.string(var_42);
    }
    #[allow(unused_mut)]
    let mut scope_43 = writer.prefix("NewDBInstanceIdentifier");
    if let Some(var_44) = &input.new_db_instance_identifier {
        scope_43.string(var_44);
    }
    #[allow(unused_mut)]
    let mut scope_45 = writer.prefix("StorageType");
    if let Some(var_46) = &input.storage_type {
        scope_45.string(var_46);
    }
    #[allow(unused_mut)]
    let mut scope_47 = writer.prefix("TdeCredentialArn");
    if let Some(var_48) = &input.tde_credential_arn {
        scope_47.string(var_48);
    }
    #[allow(unused_mut)]
    let mut scope_49 = writer.prefix("TdeCredentialPassword");
    if let Some(var_50) = &input.tde_credential_password {
        scope_49.string(var_50);
    }
    #[allow(unused_mut)]
    let mut scope_51 = writer.prefix("CACertificateIdentifier");
    if let Some(var_52) = &input.ca_certificate_identifier {
        scope_51.string(var_52);
    }
    #[allow(unused_mut)]
    let mut scope_53 = writer.prefix("Domain");
    if let Some(var_54) = &input.domain {
        scope_53.string(var_54);
    }
    #[allow(unused_mut)]
    let mut scope_55 = writer.prefix("CopyTagsToSnapshot");
    if let Some(var_56) = &input.copy_tags_to_snapshot {
        scope_55.boolean(*var_56);
    }
    #[allow(unused_mut)]
    let mut scope_57 = writer.prefix("MonitoringInterval");
    if let Some(var_58) = &input.monitoring_interval {
        scope_57.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_58).into()));
    }
    #[allow(unused_mut)]
    let mut scope_59 = writer.prefix("DBPortNumber");
    if let Some(var_60) = &input.db_port_number {
        scope_59.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_60).into()));
    }
    #[allow(unused_mut)]
    let mut scope_61 = writer.prefix("PubliclyAccessible");
    if let Some(var_62) = &input.publicly_accessible {
        scope_61.boolean(*var_62);
    }
    #[allow(unused_mut)]
    let mut scope_63 = writer.prefix("MonitoringRoleArn");
    if let Some(var_64) = &input.monitoring_role_arn {
        scope_63.string(var_64);
    }
    #[allow(unused_mut)]
    let mut scope_65 = writer.prefix("DomainIAMRoleName");
    if let Some(var_66) = &input.domain_iam_role_name {
        scope_65.string(var_66);
    }
    #[allow(unused_mut)]
    let mut scope_67 = writer.prefix("PromotionTier");
    if let Some(var_68) = &input.promotion_tier {
        scope_67.number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_68).into()));
    }
    #[allow(unused_mut)]
    let mut scope_69 = writer.prefix("EnableIAMDatabaseAuthentication");
    if let Some(var_70) = &input.enable_iam_database_authentication {
        scope_69.boolean(*var_70);
    }
    #[allow(unused_mut)]
    let mut scope_71 = writer.prefix("EnablePerformanceInsights");
    if let Some(var_72) = &input.enable_performance_insights {
        scope_71.boolean(*var_72);
    }
    #[allow(unused_mut)]
    let mut scope_73 = writer.prefix("PerformanceInsightsKMSKeyId");
    if let Some(var_74) = &input.performance_insights_kms_key_id {
        scope_73.string(var_74);
    }
    #[allow(unused_mut)]
    let mut scope_75 = writer.prefix("CloudwatchLogsExportConfiguration");
    if let Some(var_76) = &input.cloudwatch_logs_export_configuration {
        crate::protocol_serde::shape_cloudwatch_logs_export_configuration::ser_cloudwatch_logs_export_configuration(scope_75, var_76)?;
    }
    #[allow(unused_mut)]
    let mut scope_77 = writer.prefix("DeletionProtection");
    if let Some(var_78) = &input.deletion_protection {
        scope_77.boolean(*var_78);
    }
    writer.finish();
    Ok(aws_smithy_http::body::SdkBody::from(out))
}

