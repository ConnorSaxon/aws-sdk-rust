// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_advertise_byoip_cidr_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AdvertiseByoipCidrInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.cidr {
        object.key("Cidr").string(var_1.as_str());
    }
    Ok(())
}

