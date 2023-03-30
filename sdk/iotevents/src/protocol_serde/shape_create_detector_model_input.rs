// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_detector_model_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateDetectorModelInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.detector_model_definition {
        #[allow(unused_mut)]
        let mut object_2 = object.key("detectorModelDefinition").start_object();
        crate::protocol_serde::shape_detector_model_definition::ser_detector_model_definition(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.detector_model_description {
        object.key("detectorModelDescription").string(var_3.as_str());
    }
    if let Some(var_4) = &input.detector_model_name {
        object.key("detectorModelName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.evaluation_method {
        object.key("evaluationMethod").string(var_5.as_str());
    }
    if let Some(var_6) = &input.key {
        object.key("key").string(var_6.as_str());
    }
    if let Some(var_7) = &input.role_arn {
        object.key("roleArn").string(var_7.as_str());
    }
    if let Some(var_8) = &input.tags {
        let mut array_9 = object.key("tags").start_array();
        for item_10 in var_8 {
             {
                #[allow(unused_mut)]
                let mut object_11 = array_9.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_11, item_10)?;
                object_11.finish();
            }
        }
        array_9.finish();
    }
    Ok(())
}

