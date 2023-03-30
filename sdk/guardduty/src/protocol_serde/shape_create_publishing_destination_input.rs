// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_publishing_destination_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreatePublishingDestinationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.destination_properties {
        #[allow(unused_mut)]
        let mut object_3 = object.key("destinationProperties").start_object();
        crate::protocol_serde::shape_destination_properties::ser_destination_properties(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.destination_type {
        object.key("destinationType").string(var_4.as_str());
    }
    Ok(())
}

