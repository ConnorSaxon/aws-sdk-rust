// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_application_state_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeApplicationStateInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.application_id {
        object.key("ApplicationId").string(var_1.as_str());
    }
    Ok(())
}

