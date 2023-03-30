// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_register_compute_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::RegisterComputeInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.fleet_id {
        object.key("FleetId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.compute_name {
        object.key("ComputeName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.certificate_path {
        object.key("CertificatePath").string(var_3.as_str());
    }
    if let Some(var_4) = &input.dns_name {
        object.key("DnsName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.ip_address {
        object.key("IpAddress").string(var_5.as_str());
    }
    if let Some(var_6) = &input.location {
        object.key("Location").string(var_6.as_str());
    }
    Ok(())
}

