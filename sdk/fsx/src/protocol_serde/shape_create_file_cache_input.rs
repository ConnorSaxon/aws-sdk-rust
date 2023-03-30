// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_file_cache_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateFileCacheInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.file_cache_type {
        object.key("FileCacheType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.file_cache_type_version {
        object.key("FileCacheTypeVersion").string(var_3.as_str());
    }
    if let Some(var_4) = &input.storage_capacity {
        object.key("StorageCapacity").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_4).into()));
    }
    if let Some(var_5) = &input.subnet_ids {
        let mut array_6 = object.key("SubnetIds").start_array();
        for item_7 in var_5 {
             {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.security_group_ids {
        let mut array_9 = object.key("SecurityGroupIds").start_array();
        for item_10 in var_8 {
             {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    if let Some(var_11) = &input.tags {
        let mut array_12 = object.key("Tags").start_array();
        for item_13 in var_11 {
             {
                #[allow(unused_mut)]
                let mut object_14 = array_12.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_14, item_13)?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    if let Some(var_15) = &input.copy_tags_to_data_repository_associations {
        object.key("CopyTagsToDataRepositoryAssociations").boolean(*var_15);
    }
    if let Some(var_16) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_16.as_str());
    }
    if let Some(var_17) = &input.lustre_configuration {
        #[allow(unused_mut)]
        let mut object_18 = object.key("LustreConfiguration").start_object();
        crate::protocol_serde::shape_create_file_cache_lustre_configuration::ser_create_file_cache_lustre_configuration(&mut object_18, var_17)?;
        object_18.finish();
    }
    if let Some(var_19) = &input.data_repository_associations {
        let mut array_20 = object.key("DataRepositoryAssociations").start_array();
        for item_21 in var_19 {
             {
                #[allow(unused_mut)]
                let mut object_22 = array_20.value().start_object();
                crate::protocol_serde::shape_file_cache_data_repository_association::ser_file_cache_data_repository_association(&mut object_22, item_21)?;
                object_22.finish();
            }
        }
        array_20.finish();
    }
    Ok(())
}

