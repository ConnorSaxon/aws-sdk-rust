// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_fuota_task_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::StartFuotaTaskInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.lo_ra_wan {
        #[allow(unused_mut)]
        let mut object_2 = object.key("LoRaWAN").start_object();
        crate::protocol_serde::shape_lo_ra_wan_start_fuota_task::ser_lo_ra_wan_start_fuota_task(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

