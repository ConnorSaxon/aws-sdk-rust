// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_control_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateControlInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.action_plan_instructions {
        object.key("actionPlanInstructions").string(var_1.as_str());
    }
    if let Some(var_2) = &input.action_plan_title {
        object.key("actionPlanTitle").string(var_2.as_str());
    }
    if let Some(var_3) = &input.control_mapping_sources {
        let mut array_4 = object.key("controlMappingSources").start_array();
        for item_5 in var_3 {
             {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_create_control_mapping_source::ser_create_control_mapping_source(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.description {
        object.key("description").string(var_7.as_str());
    }
    if let Some(var_8) = &input.name {
        object.key("name").string(var_8.as_str());
    }
    if let Some(var_9) = &input.tags {
        #[allow(unused_mut)]
        let mut object_10 = object.key("tags").start_object();
        for (key_11, value_12) in var_9 {
             {
                object_10.key(key_11.as_str()).string(value_12.as_str());
            }
        }
        object_10.finish();
    }
    if let Some(var_13) = &input.testing_information {
        object.key("testingInformation").string(var_13.as_str());
    }
    Ok(())
}

