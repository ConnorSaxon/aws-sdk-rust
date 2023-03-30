// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_layout_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateLayoutInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.content {
        #[allow(unused_mut)]
        let mut object_2 = object.key("content").start_object();
        crate::protocol_serde::shape_layout_content::ser_layout_content(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.name {
        object.key("name").string(var_3.as_str());
    }
    Ok(())
}

