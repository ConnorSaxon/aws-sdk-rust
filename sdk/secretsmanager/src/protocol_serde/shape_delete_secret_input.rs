// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_secret_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::DeleteSecretInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.secret_id {
        object.key("SecretId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.recovery_window_in_days {
        object.key("RecoveryWindowInDays").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    if let Some(var_3) = &input.force_delete_without_recovery {
        object.key("ForceDeleteWithoutRecovery").boolean(*var_3);
    }
    Ok(())
}

