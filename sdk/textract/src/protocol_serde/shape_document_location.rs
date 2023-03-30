// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_document_location(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DocumentLocation) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.s3_object {
        #[allow(unused_mut)]
        let mut object_2 = object.key("S3Object").start_object();
        crate::protocol_serde::shape_s3_object::ser_s3_object(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

