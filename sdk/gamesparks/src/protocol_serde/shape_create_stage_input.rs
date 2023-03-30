// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_stage_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateStageInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("ClientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("Description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.role {
        object.key("Role").string(var_3.as_str());
    }
    if let Some(var_4) = &input.stage_name {
        object.key("StageName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.tags {
        #[allow(unused_mut)]
        let mut object_6 = object.key("Tags").start_object();
        for (key_7, value_8) in var_5 {
             {
                object_6.key(key_7.as_str()).string(value_8.as_str());
            }
        }
        object_6.finish();
    }
    Ok(())
}

