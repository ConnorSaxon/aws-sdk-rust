// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_vsam_attributes(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::VsamAttributes) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.format {
        object.key("format").string(var_1.as_str());
    }
    if let Some(var_2) = &input.encoding {
        object.key("encoding").string(var_2.as_str());
    }
    if input.compressed {
        object.key("compressed").boolean(input.compressed);
    }
    if let Some(var_3) = &input.primary_key {
        #[allow(unused_mut)]
        let mut object_4 = object.key("primaryKey").start_object();
        crate::protocol_serde::shape_primary_key::ser_primary_key(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.alternate_keys {
        let mut array_6 = object.key("alternateKeys").start_array();
        for item_7 in var_5 {
             {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_alternate_key::ser_alternate_key(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    Ok(())
}

