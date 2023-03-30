// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_geo_match_set_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateGeoMatchSetInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.geo_match_set_id {
        object.key("GeoMatchSetId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.change_token {
        object.key("ChangeToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.updates {
        let mut array_4 = object.key("Updates").start_array();
        for item_5 in var_3 {
             {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_geo_match_set_update::ser_geo_match_set_update(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    Ok(())
}

