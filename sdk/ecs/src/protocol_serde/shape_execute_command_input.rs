// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_execute_command_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::ExecuteCommandInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.cluster {
        object.key("cluster").string(var_1.as_str());
    }
    if let Some(var_2) = &input.container {
        object.key("container").string(var_2.as_str());
    }
    if let Some(var_3) = &input.command {
        object.key("command").string(var_3.as_str());
    }
     {
        object.key("interactive").boolean(input.interactive);
    }
    if let Some(var_4) = &input.task {
        object.key("task").string(var_4.as_str());
    }
    Ok(())
}

