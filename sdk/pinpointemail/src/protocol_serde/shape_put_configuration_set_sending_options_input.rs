// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_put_configuration_set_sending_options_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::PutConfigurationSetSendingOptionsInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.sending_enabled {
        object.key("SendingEnabled").boolean(input.sending_enabled);
    }
    Ok(())
}

