// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_secret_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::UpdateSecretInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.secret_id {
        object.key("SecretId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.client_request_token {
        object.key("ClientRequestToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.description {
        object.key("Description").string(var_3.as_str());
    }
    if let Some(var_4) = &input.kms_key_id {
        object.key("KmsKeyId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.secret_binary {
        object.key("SecretBinary").string_unchecked(&aws_smithy_types::base64::encode(var_5));
    }
    if let Some(var_6) = &input.secret_string {
        object.key("SecretString").string(var_6.as_str());
    }
    Ok(())
}

