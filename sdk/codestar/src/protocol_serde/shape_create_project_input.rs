// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_project_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateProjectInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.id {
        object.key("id").string(var_2.as_str());
    }
    if let Some(var_3) = &input.description {
        object.key("description").string(var_3.as_str());
    }
    if let Some(var_4) = &input.client_request_token {
        object.key("clientRequestToken").string(var_4.as_str());
    }
    if let Some(var_5) = &input.source_code {
        let mut array_6 = object.key("sourceCode").start_array();
        for item_7 in var_5 {
             {
                #[allow(unused_mut)]
                let mut object_8 = array_6.value().start_object();
                crate::protocol_serde::shape_code::ser_code(&mut object_8, item_7)?;
                object_8.finish();
            }
        }
        array_6.finish();
    }
    if let Some(var_9) = &input.toolchain {
        #[allow(unused_mut)]
        let mut object_10 = object.key("toolchain").start_object();
        crate::protocol_serde::shape_toolchain::ser_toolchain(&mut object_10, var_9)?;
        object_10.finish();
    }
    if let Some(var_11) = &input.tags {
        #[allow(unused_mut)]
        let mut object_12 = object.key("tags").start_object();
        for (key_13, value_14) in var_11 {
             {
                object_12.key(key_13.as_str()).string(value_14.as_str());
            }
        }
        object_12.finish();
    }
    Ok(())
}

