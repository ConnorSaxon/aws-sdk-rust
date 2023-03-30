// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_multicast_group_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateMulticastGroupInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("Description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.lo_ra_wan {
        #[allow(unused_mut)]
        let mut object_3 = object.key("LoRaWAN").start_object();
        crate::protocol_serde::shape_lo_ra_wan_multicast::ser_lo_ra_wan_multicast(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.name {
        object.key("Name").string(var_4.as_str());
    }
    Ok(())
}

