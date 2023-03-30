// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_language_model_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateLanguageModelInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.language_code {
        object.key("LanguageCode").string(var_1.as_str());
    }
    if let Some(var_2) = &input.base_model_name {
        object.key("BaseModelName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.model_name {
        object.key("ModelName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.input_data_config {
        #[allow(unused_mut)]
        let mut object_5 = object.key("InputDataConfig").start_object();
        crate::protocol_serde::shape_input_data_config::ser_input_data_config(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.tags {
        let mut array_7 = object.key("Tags").start_array();
        for item_8 in var_6 {
             {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    Ok(())
}

