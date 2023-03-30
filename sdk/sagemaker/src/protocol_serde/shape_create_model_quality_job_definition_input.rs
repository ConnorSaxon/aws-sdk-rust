// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_model_quality_job_definition_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateModelQualityJobDefinitionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.job_definition_name {
        object.key("JobDefinitionName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.model_quality_baseline_config {
        #[allow(unused_mut)]
        let mut object_3 = object.key("ModelQualityBaselineConfig").start_object();
        crate::protocol_serde::shape_model_quality_baseline_config::ser_model_quality_baseline_config(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.model_quality_app_specification {
        #[allow(unused_mut)]
        let mut object_5 = object.key("ModelQualityAppSpecification").start_object();
        crate::protocol_serde::shape_model_quality_app_specification::ser_model_quality_app_specification(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.model_quality_job_input {
        #[allow(unused_mut)]
        let mut object_7 = object.key("ModelQualityJobInput").start_object();
        crate::protocol_serde::shape_model_quality_job_input::ser_model_quality_job_input(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.model_quality_job_output_config {
        #[allow(unused_mut)]
        let mut object_9 = object.key("ModelQualityJobOutputConfig").start_object();
        crate::protocol_serde::shape_monitoring_output_config::ser_monitoring_output_config(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.job_resources {
        #[allow(unused_mut)]
        let mut object_11 = object.key("JobResources").start_object();
        crate::protocol_serde::shape_monitoring_resources::ser_monitoring_resources(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.network_config {
        #[allow(unused_mut)]
        let mut object_13 = object.key("NetworkConfig").start_object();
        crate::protocol_serde::shape_monitoring_network_config::ser_monitoring_network_config(&mut object_13, var_12)?;
        object_13.finish();
    }
    if let Some(var_14) = &input.role_arn {
        object.key("RoleArn").string(var_14.as_str());
    }
    if let Some(var_15) = &input.stopping_condition {
        #[allow(unused_mut)]
        let mut object_16 = object.key("StoppingCondition").start_object();
        crate::protocol_serde::shape_monitoring_stopping_condition::ser_monitoring_stopping_condition(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.tags {
        let mut array_18 = object.key("Tags").start_array();
        for item_19 in var_17 {
             {
                #[allow(unused_mut)]
                let mut object_20 = array_18.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_20, item_19)?;
                object_20.finish();
            }
        }
        array_18.finish();
    }
    Ok(())
}

