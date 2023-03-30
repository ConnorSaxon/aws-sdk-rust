// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_integration_response_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutIntegrationResponseInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.content_handling {
        object.key("contentHandling").string(var_1.as_str());
    }
    if let Some(var_2) = &input.response_parameters {
        #[allow(unused_mut)]
        let mut object_3 = object.key("responseParameters").start_object();
        for (key_4, value_5) in var_2 {
             {
                object_3.key(key_4.as_str()).string(value_5.as_str());
            }
        }
        object_3.finish();
    }
    if let Some(var_6) = &input.response_templates {
        #[allow(unused_mut)]
        let mut object_7 = object.key("responseTemplates").start_object();
        for (key_8, value_9) in var_6 {
             {
                object_7.key(key_8.as_str()).string(value_9.as_str());
            }
        }
        object_7.finish();
    }
    if let Some(var_10) = &input.selection_pattern {
        object.key("selectionPattern").string(var_10.as_str());
    }
    Ok(())
}

