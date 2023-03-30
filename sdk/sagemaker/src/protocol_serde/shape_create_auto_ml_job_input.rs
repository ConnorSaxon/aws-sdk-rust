// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_auto_ml_job_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateAutoMlJobInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.auto_ml_job_name {
        object.key("AutoMLJobName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.input_data_config {
        let mut array_3 = object.key("InputDataConfig").start_array();
        for item_4 in var_2 {
             {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_auto_ml_channel::ser_auto_ml_channel(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.output_data_config {
        #[allow(unused_mut)]
        let mut object_7 = object.key("OutputDataConfig").start_object();
        crate::protocol_serde::shape_auto_ml_output_data_config::ser_auto_ml_output_data_config(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.problem_type {
        object.key("ProblemType").string(var_8.as_str());
    }
    if let Some(var_9) = &input.auto_ml_job_objective {
        #[allow(unused_mut)]
        let mut object_10 = object.key("AutoMLJobObjective").start_object();
        crate::protocol_serde::shape_auto_ml_job_objective::ser_auto_ml_job_objective(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.auto_ml_job_config {
        #[allow(unused_mut)]
        let mut object_12 = object.key("AutoMLJobConfig").start_object();
        crate::protocol_serde::shape_auto_ml_job_config::ser_auto_ml_job_config(&mut object_12, var_11)?;
        object_12.finish();
    }
    if let Some(var_13) = &input.role_arn {
        object.key("RoleArn").string(var_13.as_str());
    }
    if input.generate_candidate_definitions_only {
        object.key("GenerateCandidateDefinitionsOnly").boolean(input.generate_candidate_definitions_only);
    }
    if let Some(var_14) = &input.tags {
        let mut array_15 = object.key("Tags").start_array();
        for item_16 in var_14 {
             {
                #[allow(unused_mut)]
                let mut object_17 = array_15.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_17, item_16)?;
                object_17.finish();
            }
        }
        array_15.finish();
    }
    if let Some(var_18) = &input.model_deploy_config {
        #[allow(unused_mut)]
        let mut object_19 = object.key("ModelDeployConfig").start_object();
        crate::protocol_serde::shape_model_deploy_config::ser_model_deploy_config(&mut object_19, var_18)?;
        object_19.finish();
    }
    Ok(())
}

