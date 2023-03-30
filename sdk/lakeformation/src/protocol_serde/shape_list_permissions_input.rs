// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_permissions_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ListPermissionsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.catalog_id {
        object.key("CatalogId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.include_related {
        object.key("IncludeRelated").string(var_2.as_str());
    }
    if let Some(var_3) = &input.max_results {
        object.key("MaxResults").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_3).into()));
    }
    if let Some(var_4) = &input.next_token {
        object.key("NextToken").string(var_4.as_str());
    }
    if let Some(var_5) = &input.principal {
        #[allow(unused_mut)]
        let mut object_6 = object.key("Principal").start_object();
        crate::protocol_serde::shape_data_lake_principal::ser_data_lake_principal(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.resource {
        #[allow(unused_mut)]
        let mut object_8 = object.key("Resource").start_object();
        crate::protocol_serde::shape_resource::ser_resource(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.resource_type {
        object.key("ResourceType").string(var_9.as_str());
    }
    Ok(())
}

