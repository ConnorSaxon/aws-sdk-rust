// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_partner_events_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutPartnerEventsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.entries {
        let mut array_2 = object.key("Entries").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_put_partner_events_request_entry::ser_put_partner_events_request_entry(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    Ok(())
}

