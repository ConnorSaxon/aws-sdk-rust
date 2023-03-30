// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_experiment_template_action_input_item(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::UpdateExperimentTemplateActionInputItem) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.action_id {
        object.key("actionId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.parameters {
        #[allow(unused_mut)]
        let mut object_4 = object.key("parameters").start_object();
        for (key_5, value_6) in var_3 {
             {
                object_4.key(key_5.as_str()).string(value_6.as_str());
            }
        }
        object_4.finish();
    }
    if let Some(var_7) = &input.targets {
        #[allow(unused_mut)]
        let mut object_8 = object.key("targets").start_object();
        for (key_9, value_10) in var_7 {
             {
                object_8.key(key_9.as_str()).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    if let Some(var_11) = &input.start_after {
        let mut array_12 = object.key("startAfter").start_array();
        for item_13 in var_11 {
             {
                array_12.value().string(item_13.as_str());
            }
        }
        array_12.finish();
    }
    Ok(())
}

