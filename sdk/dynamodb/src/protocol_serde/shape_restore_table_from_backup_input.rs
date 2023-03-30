// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_restore_table_from_backup_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RestoreTableFromBackupInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.target_table_name {
        object.key("TargetTableName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.backup_arn {
        object.key("BackupArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.billing_mode_override {
        object.key("BillingModeOverride").string(var_3.as_str());
    }
    if let Some(var_4) = &input.global_secondary_index_override {
        let mut array_5 = object.key("GlobalSecondaryIndexOverride").start_array();
        for item_6 in var_4 {
             {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_global_secondary_index::ser_global_secondary_index(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.local_secondary_index_override {
        let mut array_9 = object.key("LocalSecondaryIndexOverride").start_array();
        for item_10 in var_8 {
             {
                #[allow(unused_mut)]
                let mut object_11 = array_9.value().start_object();
                crate::protocol_serde::shape_local_secondary_index::ser_local_secondary_index(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    if let Some(var_12) = &input.provisioned_throughput_override {
        #[allow(unused_mut)]
        let mut object_13 = object.key("ProvisionedThroughputOverride").start_object();
        crate::protocol_serde::shape_provisioned_throughput::ser_provisioned_throughput(&mut object_13, var_12)?;
        object_13.finish();
    }
    if let Some(var_14) = &input.sse_specification_override {
        #[allow(unused_mut)]
        let mut object_15 = object.key("SSESpecificationOverride").start_object();
        crate::protocol_serde::shape_sse_specification::ser_sse_specification(&mut object_15, var_14)?;
        object_15.finish();
    }
    Ok(())
}

