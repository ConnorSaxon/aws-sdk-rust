// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_mqtt_context(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MqttContext) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.username {
        object.key("username").string(var_1.as_str());
    }
    if let Some(var_2) = &input.password {
        object.key("password").string_unchecked(&aws_smithy_types::base64::encode(var_2));
    }
    if let Some(var_3) = &input.client_id {
        object.key("clientId").string(var_3.as_str());
    }
    Ok(())
}

