// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_fraudster_registration_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartFraudsterRegistrationJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("ClientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.job_name {
        object.key("JobName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.domain_id {
        object.key("DomainId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.data_access_role_arn {
        object.key("DataAccessRoleArn").string(var_4.as_str());
    }
    if let Some(var_5) = &input.registration_config {
        #[allow(unused_mut)]
        let mut object_6 = object.key("RegistrationConfig").start_object();
        crate::protocol_serde::shape_registration_config::ser_registration_config(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.input_data_config {
        #[allow(unused_mut)]
        let mut object_8 = object.key("InputDataConfig").start_object();
        crate::protocol_serde::shape_input_data_config::ser_input_data_config(&mut object_8, var_7)?;
        object_8.finish();
    }
    if let Some(var_9) = &input.output_data_config {
        #[allow(unused_mut)]
        let mut object_10 = object.key("OutputDataConfig").start_object();
        crate::protocol_serde::shape_output_data_config::ser_output_data_config(&mut object_10, var_9)?;
        object_10.finish();
    }
    Ok(())
}

