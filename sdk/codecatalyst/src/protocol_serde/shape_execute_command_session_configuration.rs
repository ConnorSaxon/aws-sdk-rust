// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_execute_command_session_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ExecuteCommandSessionConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.command {
        object.key("command").string(var_1.as_str());
    }
    if let Some(var_2) = &input.arguments {
        let mut array_3 = object.key("arguments").start_array();
        for item_4 in var_2 {
             {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    Ok(())
}

