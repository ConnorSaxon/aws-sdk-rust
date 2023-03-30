// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_fleet_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateFleetInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.fleet_id {
        object.key("fleetId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.signal_catalog_arn {
        object.key("signalCatalogArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.tags {
        let mut array_5 = object.key("tags").start_array();
        for item_6 in var_4 {
             {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    Ok(())
}

