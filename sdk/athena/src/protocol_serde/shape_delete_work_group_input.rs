// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_work_group_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteWorkGroupInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.work_group {
        object.key("WorkGroup").string(var_1.as_str());
    }
    if let Some(var_2) = &input.recursive_delete_option {
        object.key("RecursiveDeleteOption").boolean(*var_2);
    }
    Ok(())
}

