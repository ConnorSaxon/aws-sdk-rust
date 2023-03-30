// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_answer_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateAnswerInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.choice_updates {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ChoiceUpdates").start_object();
        for (key_3, value_4) in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_5 = object_2.key(key_3.as_str()).start_object();
                crate::protocol_serde::shape_choice_update::ser_choice_update(&mut object_5, value_4)?;
                object_5.finish();
            }
        }
        object_2.finish();
    }
    if input.is_applicable {
        object.key("IsApplicable").boolean(input.is_applicable);
    }
    if let Some(var_6) = &input.notes {
        object.key("Notes").string(var_6.as_str());
    }
    if let Some(var_7) = &input.reason {
        object.key("Reason").string(var_7.as_str());
    }
    if let Some(var_8) = &input.selected_choices {
        let mut array_9 = object.key("SelectedChoices").start_array();
        for item_10 in var_8 {
             {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    Ok(())
}

