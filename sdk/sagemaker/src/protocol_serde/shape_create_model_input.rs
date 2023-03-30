// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_model_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateModelInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.model_name {
        object.key("ModelName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.primary_container {
        #[allow(unused_mut)]
        let mut object_3 = object.key("PrimaryContainer").start_object();
        crate::protocol_serde::shape_container_definition::ser_container_definition(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.containers {
        let mut array_5 = object.key("Containers").start_array();
        for item_6 in var_4 {
             {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_container_definition::ser_container_definition(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.inference_execution_config {
        #[allow(unused_mut)]
        let mut object_9 = object.key("InferenceExecutionConfig").start_object();
        crate::protocol_serde::shape_inference_execution_config::ser_inference_execution_config(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.execution_role_arn {
        object.key("ExecutionRoleArn").string(var_10.as_str());
    }
    if let Some(var_11) = &input.tags {
        let mut array_12 = object.key("Tags").start_array();
        for item_13 in var_11 {
             {
                #[allow(unused_mut)]
                let mut object_14 = array_12.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_14, item_13)?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    if let Some(var_15) = &input.vpc_config {
        #[allow(unused_mut)]
        let mut object_16 = object.key("VpcConfig").start_object();
        crate::protocol_serde::shape_vpc_config::ser_vpc_config(&mut object_16, var_15)?;
        object_16.finish();
    }
    if input.enable_network_isolation {
        object.key("EnableNetworkIsolation").boolean(input.enable_network_isolation);
    }
    Ok(())
}

