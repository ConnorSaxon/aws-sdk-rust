// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_associate_client_device_with_core_device_entry(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AssociateClientDeviceWithCoreDeviceEntry) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.thing_name {
        object.key("thingName").string(var_1.as_str());
    }
    Ok(())
}

