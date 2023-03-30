// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_workspace_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateWorkspaceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.alias {
        object.key("alias").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_token {
        object.key("clientToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.tags {
        #[allow(unused_mut)]
        let mut object_4 = object.key("tags").start_object();
        for (key_5, value_6) in var_3 {
             {
                object_4.key(key_5.as_str()).string(value_6.as_str());
            }
        }
        object_4.finish();
    }
    Ok(())
}

