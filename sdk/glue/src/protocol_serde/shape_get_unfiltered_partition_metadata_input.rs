// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_unfiltered_partition_metadata_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetUnfilteredPartitionMetadataInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.catalog_id {
        object.key("CatalogId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.database_name {
        object.key("DatabaseName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.table_name {
        object.key("TableName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.partition_values {
        let mut array_5 = object.key("PartitionValues").start_array();
        for item_6 in var_4 {
             {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.audit_context {
        #[allow(unused_mut)]
        let mut object_8 = object.key("AuditContext").start_object();
        crate::protocol_serde::shape_audit_context::ser_audit_context(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.supported_permission_types {
        let mut array_10 = object.key("SupportedPermissionTypes").start_array();
        for item_11 in var_9 {
             {
                array_10.value().string(item_11.as_str());
            }
        }
        array_10.finish();
    }
    Ok(())
}

