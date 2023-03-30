// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_smb_file_share_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateSmbFileShareInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.file_share_arn {
        object.key("FileShareARN").string(var_1.as_str());
    }
    if let Some(var_2) = &input.kms_encrypted {
        object.key("KMSEncrypted").boolean(*var_2);
    }
    if let Some(var_3) = &input.kms_key {
        object.key("KMSKey").string(var_3.as_str());
    }
    if let Some(var_4) = &input.default_storage_class {
        object.key("DefaultStorageClass").string(var_4.as_str());
    }
    if let Some(var_5) = &input.object_acl {
        object.key("ObjectACL").string(var_5.as_str());
    }
    if let Some(var_6) = &input.read_only {
        object.key("ReadOnly").boolean(*var_6);
    }
    if let Some(var_7) = &input.guess_mime_type_enabled {
        object.key("GuessMIMETypeEnabled").boolean(*var_7);
    }
    if let Some(var_8) = &input.requester_pays {
        object.key("RequesterPays").boolean(*var_8);
    }
    if let Some(var_9) = &input.smbacl_enabled {
        object.key("SMBACLEnabled").boolean(*var_9);
    }
    if let Some(var_10) = &input.access_based_enumeration {
        object.key("AccessBasedEnumeration").boolean(*var_10);
    }
    if let Some(var_11) = &input.admin_user_list {
        let mut array_12 = object.key("AdminUserList").start_array();
        for item_13 in var_11 {
             {
                array_12.value().string(item_13.as_str());
            }
        }
        array_12.finish();
    }
    if let Some(var_14) = &input.valid_user_list {
        let mut array_15 = object.key("ValidUserList").start_array();
        for item_16 in var_14 {
             {
                array_15.value().string(item_16.as_str());
            }
        }
        array_15.finish();
    }
    if let Some(var_17) = &input.invalid_user_list {
        let mut array_18 = object.key("InvalidUserList").start_array();
        for item_19 in var_17 {
             {
                array_18.value().string(item_19.as_str());
            }
        }
        array_18.finish();
    }
    if let Some(var_20) = &input.audit_destination_arn {
        object.key("AuditDestinationARN").string(var_20.as_str());
    }
    if let Some(var_21) = &input.case_sensitivity {
        object.key("CaseSensitivity").string(var_21.as_str());
    }
    if let Some(var_22) = &input.file_share_name {
        object.key("FileShareName").string(var_22.as_str());
    }
    if let Some(var_23) = &input.cache_attributes {
        #[allow(unused_mut)]
        let mut object_24 = object.key("CacheAttributes").start_object();
        crate::protocol_serde::shape_cache_attributes::ser_cache_attributes(&mut object_24, var_23)?;
        object_24.finish();
    }
    if let Some(var_25) = &input.notification_policy {
        object.key("NotificationPolicy").string(var_25.as_str());
    }
    if let Some(var_26) = &input.oplocks_enabled {
        object.key("OplocksEnabled").boolean(*var_26);
    }
    Ok(())
}

