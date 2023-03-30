// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_send_data_to_wireless_device_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::SendDataToWirelessDeviceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.payload_data {
        object.key("PayloadData").string(var_1.as_str());
    }
    if let Some(var_2) = &input.transmit_mode {
        object.key("TransmitMode").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    if let Some(var_3) = &input.wireless_metadata {
        #[allow(unused_mut)]
        let mut object_4 = object.key("WirelessMetadata").start_object();
        crate::protocol_serde::shape_wireless_metadata::ser_wireless_metadata(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

