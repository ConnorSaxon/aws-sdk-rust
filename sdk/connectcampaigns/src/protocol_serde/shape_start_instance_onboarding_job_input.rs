// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_instance_onboarding_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartInstanceOnboardingJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.encryption_config {
        #[allow(unused_mut)]
        let mut object_2 = object.key("encryptionConfig").start_object();
        crate::protocol_serde::shape_encryption_config::ser_encryption_config(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

