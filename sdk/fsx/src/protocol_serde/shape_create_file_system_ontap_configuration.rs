// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_file_system_ontap_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::CreateFileSystemOntapConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.automatic_backup_retention_days {
        object.key("AutomaticBackupRetentionDays").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_1).into()));
    }
    if let Some(var_2) = &input.daily_automatic_backup_start_time {
        object.key("DailyAutomaticBackupStartTime").string(var_2.as_str());
    }
    if let Some(var_3) = &input.deployment_type {
        object.key("DeploymentType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.endpoint_ip_address_range {
        object.key("EndpointIpAddressRange").string(var_4.as_str());
    }
    if let Some(var_5) = &input.fsx_admin_password {
        object.key("FsxAdminPassword").string(var_5.as_str());
    }
    if let Some(var_6) = &input.disk_iops_configuration {
        #[allow(unused_mut)]
        let mut object_7 = object.key("DiskIopsConfiguration").start_object();
        crate::protocol_serde::shape_disk_iops_configuration::ser_disk_iops_configuration(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.preferred_subnet_id {
        object.key("PreferredSubnetId").string(var_8.as_str());
    }
    if let Some(var_9) = &input.route_table_ids {
        let mut array_10 = object.key("RouteTableIds").start_array();
        for item_11 in var_9 {
             {
                array_10.value().string(item_11.as_str());
            }
        }
        array_10.finish();
    }
    if let Some(var_12) = &input.throughput_capacity {
        object.key("ThroughputCapacity").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_12).into()));
    }
    if let Some(var_13) = &input.weekly_maintenance_start_time {
        object.key("WeeklyMaintenanceStartTime").string(var_13.as_str());
    }
    Ok(())
}

