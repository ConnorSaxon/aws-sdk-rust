// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_listener_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeListenerInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.listener_arn {
        object.key("ListenerArn").string(var_1.as_str());
    }
    Ok(())
}

