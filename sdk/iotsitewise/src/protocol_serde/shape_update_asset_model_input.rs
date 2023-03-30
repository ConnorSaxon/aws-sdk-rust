// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_asset_model_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateAssetModelInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.asset_model_composite_models {
        let mut array_2 = object.key("assetModelCompositeModels").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_asset_model_composite_model::ser_asset_model_composite_model(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.asset_model_description {
        object.key("assetModelDescription").string(var_5.as_str());
    }
    if let Some(var_6) = &input.asset_model_hierarchies {
        let mut array_7 = object.key("assetModelHierarchies").start_array();
        for item_8 in var_6 {
             {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_asset_model_hierarchy::ser_asset_model_hierarchy(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.asset_model_name {
        object.key("assetModelName").string(var_10.as_str());
    }
    if let Some(var_11) = &input.asset_model_properties {
        let mut array_12 = object.key("assetModelProperties").start_array();
        for item_13 in var_11 {
             {
                #[allow(unused_mut)]
                let mut object_14 = array_12.value().start_object();
                crate::protocol_serde::shape_asset_model_property::ser_asset_model_property(&mut object_14, item_13)?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    if let Some(var_15) = &input.client_token {
        object.key("clientToken").string(var_15.as_str());
    }
    Ok(())
}

