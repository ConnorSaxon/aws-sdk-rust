// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_restore_cluster_from_snapshot_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RestoreClusterFromSnapshotInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.cluster_name {
        object.key("clusterName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.kms_key_id {
        object.key("kmsKeyId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.subnet_ids {
        let mut array_4 = object.key("subnetIds").start_array();
        for item_5 in var_3 {
             {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.tags {
        #[allow(unused_mut)]
        let mut object_7 = object.key("tags").start_object();
        for (key_8, value_9) in var_6 {
             {
                object_7.key(key_8.as_str()).string(value_9.as_str());
            }
        }
        object_7.finish();
    }
    if let Some(var_10) = &input.vpc_security_group_ids {
        let mut array_11 = object.key("vpcSecurityGroupIds").start_array();
        for item_12 in var_10 {
             {
                array_11.value().string(item_12.as_str());
            }
        }
        array_11.finish();
    }
    Ok(())
}

