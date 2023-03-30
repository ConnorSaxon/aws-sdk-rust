// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_profile_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateProfileJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Configuration").start_object();
        crate::protocol_serde::shape_profile_configuration::ser_profile_configuration(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.encryption_key_arn {
        object.key("EncryptionKeyArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.encryption_mode {
        object.key("EncryptionMode").string(var_4.as_str());
    }
    if let Some(var_5) = &input.job_sample {
        #[allow(unused_mut)]
        let mut object_6 = object.key("JobSample").start_object();
        crate::protocol_serde::shape_job_sample::ser_job_sample(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.log_subscription {
        object.key("LogSubscription").string(var_7.as_str());
    }
    if input.max_capacity != 0 {
        object.key("MaxCapacity").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_capacity).into()));
    }
    if input.max_retries != 0 {
        object.key("MaxRetries").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.max_retries).into()));
    }
    if let Some(var_8) = &input.output_location {
        #[allow(unused_mut)]
        let mut object_9 = object.key("OutputLocation").start_object();
        crate::protocol_serde::shape_s3_location::ser_s3_location(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.role_arn {
        object.key("RoleArn").string(var_10.as_str());
    }
    if input.timeout != 0 {
        object.key("Timeout").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.timeout).into()));
    }
    if let Some(var_11) = &input.validation_configurations {
        let mut array_12 = object.key("ValidationConfigurations").start_array();
        for item_13 in var_11 {
             {
                #[allow(unused_mut)]
                let mut object_14 = array_12.value().start_object();
                crate::protocol_serde::shape_validation_configuration::ser_validation_configuration(&mut object_14, item_13)?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    Ok(())
}

