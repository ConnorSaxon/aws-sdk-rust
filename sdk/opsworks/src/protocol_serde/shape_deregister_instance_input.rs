// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_deregister_instance_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeregisterInstanceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.instance_id {
        object.key("InstanceId").string(var_1.as_str());
    }
    Ok(())
}

