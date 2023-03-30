// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_invoke_screen_automation_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::InvokeScreenAutomationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_request_token {
        object.key("clientRequestToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.row_id {
        object.key("rowId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.variables {
        #[allow(unused_mut)]
        let mut object_4 = object.key("variables").start_object();
        for (key_5, value_6) in var_3 {
             {
                #[allow(unused_mut)]
                let mut object_7 = object_4.key(key_5.as_str()).start_object();
                crate::protocol_serde::shape_variable_value::ser_variable_value(&mut object_7, value_6)?;
                object_7.finish();
            }
        }
        object_4.finish();
    }
    Ok(())
}

