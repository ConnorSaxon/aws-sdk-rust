// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_generated_code_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartGeneratedCodeJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.generator {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Generator").start_object();
        crate::protocol_serde::shape_generator::ser_generator(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

