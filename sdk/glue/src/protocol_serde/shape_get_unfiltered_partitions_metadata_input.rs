// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_unfiltered_partitions_metadata_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetUnfilteredPartitionsMetadataInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.catalog_id {
        object.key("CatalogId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.database_name {
        object.key("DatabaseName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.table_name {
        object.key("TableName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.expression {
        object.key("Expression").string(var_4.as_str());
    }
    if let Some(var_5) = &input.audit_context {
        #[allow(unused_mut)]
        let mut object_6 = object.key("AuditContext").start_object();
        crate::protocol_serde::shape_audit_context::ser_audit_context(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.supported_permission_types {
        let mut array_8 = object.key("SupportedPermissionTypes").start_array();
        for item_9 in var_7 {
             {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    if let Some(var_10) = &input.next_token {
        object.key("NextToken").string(var_10.as_str());
    }
    if let Some(var_11) = &input.segment {
        #[allow(unused_mut)]
        let mut object_12 = object.key("Segment").start_object();
        crate::protocol_serde::shape_segment::ser_segment(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_13).into()));
    }
    Ok(())
}

