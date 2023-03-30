// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_object_attribute_range(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ObjectAttributeRange) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.attribute_key {
        #[allow(unused_mut)]
        let mut object_2 = object.key("AttributeKey").start_object();
        crate::protocol_serde::shape_attribute_key::ser_attribute_key(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.range {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Range").start_object();
        crate::protocol_serde::shape_typed_attribute_value_range::ser_typed_attribute_value_range(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

