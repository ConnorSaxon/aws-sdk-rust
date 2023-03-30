// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_batch_attach_object(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::BatchAttachObject) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.parent_reference {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ParentReference").start_object();
        crate::protocol_serde::shape_object_reference::ser_object_reference(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.child_reference {
        #[allow(unused_mut)]
        let mut object_4 = object.key("ChildReference").start_object();
        crate::protocol_serde::shape_object_reference::ser_object_reference(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.link_name {
        object.key("LinkName").string(var_5.as_str());
    }
    Ok(())
}

