// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_runtime_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeRuntimeConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.fleet_id {
        object.key("FleetId").string(var_1.as_str());
    }
    Ok(())
}

