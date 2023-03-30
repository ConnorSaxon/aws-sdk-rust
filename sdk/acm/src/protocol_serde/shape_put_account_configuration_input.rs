// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_account_configuration_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutAccountConfigurationInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.expiry_events {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ExpiryEvents").start_object();
        crate::protocol_serde::shape_expiry_events_configuration::ser_expiry_events_configuration(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.idempotency_token {
        object.key("IdempotencyToken").string(var_3.as_str());
    }
    Ok(())
}

