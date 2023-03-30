// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_pipeline_definition_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutPipelineDefinitionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.pipeline_id {
        object.key("pipelineId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.pipeline_objects {
        let mut array_3 = object.key("pipelineObjects").start_array();
        for item_4 in var_2 {
             {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_pipeline_object::ser_pipeline_object(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.parameter_objects {
        let mut array_7 = object.key("parameterObjects").start_array();
        for item_8 in var_6 {
             {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_parameter_object::ser_parameter_object(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.parameter_values {
        let mut array_11 = object.key("parameterValues").start_array();
        for item_12 in var_10 {
             {
                #[allow(unused_mut)]
                let mut object_13 = array_11.value().start_object();
                crate::protocol_serde::shape_parameter_value::ser_parameter_value(&mut object_13, item_12)?;
                object_13.finish();
            }
        }
        array_11.finish();
    }
    Ok(())
}

