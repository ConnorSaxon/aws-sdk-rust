// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_set_user_mfa_preference_input(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::input::SetUserMfaPreferenceInput) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.sms_mfa_settings {
        #[allow(unused_mut)]
        let mut object_2 = object.key("SMSMfaSettings").start_object();
        crate::protocol_serde::shape_sms_mfa_settings_type::ser_sms_mfa_settings_type(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.software_token_mfa_settings {
        #[allow(unused_mut)]
        let mut object_4 = object.key("SoftwareTokenMfaSettings").start_object();
        crate::protocol_serde::shape_software_token_mfa_settings_type::ser_software_token_mfa_settings_type(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.access_token {
        object.key("AccessToken").string(var_5.as_str());
    }
    Ok(())
}

