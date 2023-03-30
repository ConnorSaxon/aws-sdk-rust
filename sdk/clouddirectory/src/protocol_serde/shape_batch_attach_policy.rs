// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_batch_attach_policy(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::BatchAttachPolicy) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.policy_reference {
        #[allow(unused_mut)]
        let mut object_2 = object.key("PolicyReference").start_object();
        crate::protocol_serde::shape_object_reference::ser_object_reference(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.object_reference {
        #[allow(unused_mut)]
        let mut object_4 = object.key("ObjectReference").start_object();
        crate::protocol_serde::shape_object_reference::ser_object_reference(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

