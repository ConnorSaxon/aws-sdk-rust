// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_virtual_machine_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::GetVirtualMachineInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.resource_arn {
        object.key("ResourceArn").string(var_1.as_str());
    }
    Ok(())
}

