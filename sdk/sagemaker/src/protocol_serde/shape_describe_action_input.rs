// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_action_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeActionInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.action_name {
        object.key("ActionName").string(var_1.as_str());
    }
    Ok(())
}

