// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disassociate_phone_numbers_from_voice_connector_group_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DisassociatePhoneNumbersFromVoiceConnectorGroupInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.e164_phone_numbers {
        let mut array_2 = object.key("E164PhoneNumbers").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    Ok(())
}

