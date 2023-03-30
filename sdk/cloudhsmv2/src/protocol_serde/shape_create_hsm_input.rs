// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_hsm_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateHsmInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.cluster_id {
        object.key("ClusterId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.availability_zone {
        object.key("AvailabilityZone").string(var_2.as_str());
    }
    if let Some(var_3) = &input.ip_address {
        object.key("IpAddress").string(var_3.as_str());
    }
    Ok(())
}

