// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_component_data(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::UpdateComponentData) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.id {
        object.key("id").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.source_id {
        object.key("sourceId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.component_type {
        object.key("componentType").string(var_4.as_str());
    }
    if let Some(var_5) = &input.properties {
        #[allow(unused_mut)]
        let mut object_6 = object.key("properties").start_object();
        for (key_7, value_8) in var_5 {
             {
                #[allow(unused_mut)]
                let mut object_9 = object_6.key(key_7.as_str()).start_object();
                crate::protocol_serde::shape_component_property::ser_component_property(&mut object_9, value_8)?;
                object_9.finish();
            }
        }
        object_6.finish();
    }
    if let Some(var_10) = &input.children {
        let mut array_11 = object.key("children").start_array();
        for item_12 in var_10 {
             {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_component_child::ser_component_child(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    if let Some(var_14) = &input.variants {
        let mut array_15 = object.key("variants").start_array();
        for item_16 in var_14 {
             {
                #[allow(unused_mut)]
                let mut object_17 = array_15.value().start_object();
                crate::protocol_serde::shape_component_variant::ser_component_variant(&mut object_17, item_16)?;
                object_17.finish();
            }
        }
        array_15.finish();
    }
    if let Some(var_18) = &input.overrides {
        #[allow(unused_mut)]
        let mut object_19 = object.key("overrides").start_object();
        for (key_20, value_21) in var_18 {
             {
                #[allow(unused_mut)]
                let mut object_22 = object_19.key(key_20.as_str()).start_object();
                for (key_23, value_24) in value_21 {
                     {
                        object_22.key(key_23.as_str()).string(value_24.as_str());
                    }
                }
                object_22.finish();
            }
        }
        object_19.finish();
    }
    if let Some(var_25) = &input.binding_properties {
        #[allow(unused_mut)]
        let mut object_26 = object.key("bindingProperties").start_object();
        for (key_27, value_28) in var_25 {
             {
                #[allow(unused_mut)]
                let mut object_29 = object_26.key(key_27.as_str()).start_object();
                crate::protocol_serde::shape_component_binding_properties_value::ser_component_binding_properties_value(&mut object_29, value_28)?;
                object_29.finish();
            }
        }
        object_26.finish();
    }
    if let Some(var_30) = &input.collection_properties {
        #[allow(unused_mut)]
        let mut object_31 = object.key("collectionProperties").start_object();
        for (key_32, value_33) in var_30 {
             {
                #[allow(unused_mut)]
                let mut object_34 = object_31.key(key_32.as_str()).start_object();
                crate::protocol_serde::shape_component_data_configuration::ser_component_data_configuration(&mut object_34, value_33)?;
                object_34.finish();
            }
        }
        object_31.finish();
    }
    if let Some(var_35) = &input.events {
        #[allow(unused_mut)]
        let mut object_36 = object.key("events").start_object();
        for (key_37, value_38) in var_35 {
             {
                #[allow(unused_mut)]
                let mut object_39 = object_36.key(key_37.as_str()).start_object();
                crate::protocol_serde::shape_component_event::ser_component_event(&mut object_39, value_38)?;
                object_39.finish();
            }
        }
        object_36.finish();
    }
    if let Some(var_40) = &input.schema_version {
        object.key("schemaVersion").string(var_40.as_str());
    }
    Ok(())
}

