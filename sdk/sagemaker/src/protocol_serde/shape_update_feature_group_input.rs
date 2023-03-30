// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_feature_group_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateFeatureGroupInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.feature_group_name {
        object.key("FeatureGroupName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.feature_additions {
        let mut array_3 = object.key("FeatureAdditions").start_array();
        for item_4 in var_2 {
             {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_feature_definition::ser_feature_definition(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    Ok(())
}

