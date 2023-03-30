// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_image(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Image) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.id {
        object.key("id").string(var_1.as_str());
    }
    if let Some(var_2) = &input.file {
        #[allow(unused_mut)]
        let mut object_3 = object.key("file").start_object();
        crate::protocol_serde::shape_image_file::ser_image_file(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}

