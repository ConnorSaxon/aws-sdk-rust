// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_project_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteProjectInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.arn {
        object.key("arn").string(var_1.as_str());
    }
    Ok(())
}

