// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_associate_wireless_device_with_thing_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::AssociateWirelessDeviceWithThingInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.thing_arn {
        object.key("ThingArn").string(var_1.as_str());
    }
    Ok(())
}

