// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_local_secondary_index(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::LocalSecondaryIndex) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.index_name {
        object.key("IndexName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.key_schema {
        let mut array_3 = object.key("KeySchema").start_array();
        for item_4 in var_2 {
             {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_key_schema_element::ser_key_schema_element(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.projection {
        #[allow(unused_mut)]
        let mut object_7 = object.key("Projection").start_object();
        crate::protocol_serde::shape_projection::ser_projection(&mut object_7, var_6)?;
        object_7.finish();
    }
    Ok(())
}

