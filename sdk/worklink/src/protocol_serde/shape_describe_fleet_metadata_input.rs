// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_fleet_metadata_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DescribeFleetMetadataInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.fleet_arn {
        object.key("FleetArn").string(var_1.as_str());
    }
    Ok(())
}

