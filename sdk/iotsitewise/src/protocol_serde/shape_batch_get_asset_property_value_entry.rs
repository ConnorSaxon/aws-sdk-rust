// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_batch_get_asset_property_value_entry(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::BatchGetAssetPropertyValueEntry) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.entry_id {
        object.key("entryId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.asset_id {
        object.key("assetId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.property_id {
        object.key("propertyId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.property_alias {
        object.key("propertyAlias").string(var_4.as_str());
    }
    Ok(())
}

