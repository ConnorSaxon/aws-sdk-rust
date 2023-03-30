// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_link_attribute_action(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::LinkAttributeAction) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.attribute_action_type {
        object.key("AttributeActionType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.attribute_update_value {
        #[allow(unused_mut)]
        let mut object_3 = object.key("AttributeUpdateValue").start_object();
        crate::protocol_serde::shape_typed_attribute_value::ser_typed_attribute_value(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}

