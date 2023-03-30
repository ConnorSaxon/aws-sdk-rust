// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_asset_model_composite_model_definition(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AssetModelCompositeModelDefinition) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.r#type {
        object.key("type").string(var_3.as_str());
    }
    if let Some(var_4) = &input.properties {
        let mut array_5 = object.key("properties").start_array();
        for item_6 in var_4 {
             {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_asset_model_property_definition::ser_asset_model_property_definition(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    Ok(())
}

