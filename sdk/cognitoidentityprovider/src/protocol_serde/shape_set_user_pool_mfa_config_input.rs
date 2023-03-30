// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_set_user_pool_mfa_config_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::SetUserPoolMfaConfigInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.user_pool_id {
        object.key("UserPoolId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.sms_mfa_configuration {
        #[allow(unused_mut)]
        let mut object_3 = object.key("SmsMfaConfiguration").start_object();
        crate::protocol_serde::shape_sms_mfa_config_type::ser_sms_mfa_config_type(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.software_token_mfa_configuration {
        #[allow(unused_mut)]
        let mut object_5 = object.key("SoftwareTokenMfaConfiguration").start_object();
        crate::protocol_serde::shape_software_token_mfa_config_type::ser_software_token_mfa_config_type(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.mfa_configuration {
        object.key("MfaConfiguration").string(var_6.as_str());
    }
    Ok(())
}

