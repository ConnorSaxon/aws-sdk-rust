// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_address_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::CreateAddressInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.address {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Address").start_object();
        crate::protocol_serde::shape_address::ser_address(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

