// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_task_set_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateTaskSetInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.cluster {
        object.key("cluster").string(var_1.as_str());
    }
    if let Some(var_2) = &input.service {
        object.key("service").string(var_2.as_str());
    }
    if let Some(var_3) = &input.task_set {
        object.key("taskSet").string(var_3.as_str());
    }
    if let Some(var_4) = &input.scale {
        #[allow(unused_mut)]
        let mut object_5 = object.key("scale").start_object();
        crate::protocol_serde::shape_scale::ser_scale(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}

