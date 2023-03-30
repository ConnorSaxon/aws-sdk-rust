// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_entity_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateEntityInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.components {
        #[allow(unused_mut)]
        let mut object_2 = object.key("components").start_object();
        for (key_3, value_4) in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_5 = object_2.key(key_3.as_str()).start_object();
                crate::protocol_serde::shape_component_request::ser_component_request(&mut object_5, value_4)?;
                object_5.finish();
            }
        }
        object_2.finish();
    }
    if let Some(var_6) = &input.description {
        object.key("description").string(var_6.as_str());
    }
    if let Some(var_7) = &input.entity_id {
        object.key("entityId").string(var_7.as_str());
    }
    if let Some(var_8) = &input.entity_name {
        object.key("entityName").string(var_8.as_str());
    }
    if let Some(var_9) = &input.parent_entity_id {
        object.key("parentEntityId").string(var_9.as_str());
    }
    if let Some(var_10) = &input.tags {
        #[allow(unused_mut)]
        let mut object_11 = object.key("tags").start_object();
        for (key_12, value_13) in var_10 {
             {
                object_11.key(key_12.as_str()).string(value_13.as_str());
            }
        }
        object_11.finish();
    }
    Ok(())
}

