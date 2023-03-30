// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_server_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateServerInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.associate_public_ip_address {
        object.key("AssociatePublicIpAddress").boolean(*var_1);
    }
    if let Some(var_2) = &input.custom_domain {
        object.key("CustomDomain").string(var_2.as_str());
    }
    if let Some(var_3) = &input.custom_certificate {
        object.key("CustomCertificate").string(var_3.as_str());
    }
    if let Some(var_4) = &input.custom_private_key {
        object.key("CustomPrivateKey").string(var_4.as_str());
    }
    if let Some(var_5) = &input.disable_automated_backup {
        object.key("DisableAutomatedBackup").boolean(*var_5);
    }
    if let Some(var_6) = &input.engine {
        object.key("Engine").string(var_6.as_str());
    }
    if let Some(var_7) = &input.engine_model {
        object.key("EngineModel").string(var_7.as_str());
    }
    if let Some(var_8) = &input.engine_version {
        object.key("EngineVersion").string(var_8.as_str());
    }
    if let Some(var_9) = &input.engine_attributes {
        let mut array_10 = object.key("EngineAttributes").start_array();
        for item_11 in var_9 {
             {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::protocol_serde::shape_engine_attribute::ser_engine_attribute(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    if let Some(var_13) = &input.backup_retention_count {
        object.key("BackupRetentionCount").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_13).into()));
    }
    if let Some(var_14) = &input.server_name {
        object.key("ServerName").string(var_14.as_str());
    }
    if let Some(var_15) = &input.instance_profile_arn {
        object.key("InstanceProfileArn").string(var_15.as_str());
    }
    if let Some(var_16) = &input.instance_type {
        object.key("InstanceType").string(var_16.as_str());
    }
    if let Some(var_17) = &input.key_pair {
        object.key("KeyPair").string(var_17.as_str());
    }
    if let Some(var_18) = &input.preferred_maintenance_window {
        object.key("PreferredMaintenanceWindow").string(var_18.as_str());
    }
    if let Some(var_19) = &input.preferred_backup_window {
        object.key("PreferredBackupWindow").string(var_19.as_str());
    }
    if let Some(var_20) = &input.security_group_ids {
        let mut array_21 = object.key("SecurityGroupIds").start_array();
        for item_22 in var_20 {
             {
                array_21.value().string(item_22.as_str());
            }
        }
        array_21.finish();
    }
    if let Some(var_23) = &input.service_role_arn {
        object.key("ServiceRoleArn").string(var_23.as_str());
    }
    if let Some(var_24) = &input.subnet_ids {
        let mut array_25 = object.key("SubnetIds").start_array();
        for item_26 in var_24 {
             {
                array_25.value().string(item_26.as_str());
            }
        }
        array_25.finish();
    }
    if let Some(var_27) = &input.tags {
        let mut array_28 = object.key("Tags").start_array();
        for item_29 in var_27 {
             {
                #[allow(unused_mut)]
                let mut object_30 = array_28.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_30, item_29)?;
                object_30.finish();
            }
        }
        array_28.finish();
    }
    if let Some(var_31) = &input.backup_id {
        object.key("BackupId").string(var_31.as_str());
    }
    Ok(())
}

