// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_ios_import_client_branding_attributes(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::IosImportClientBrandingAttributes) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.logo {
        object.key("Logo").string_unchecked(&aws_smithy_types::base64::encode(var_1));
    }
    if let Some(var_2) = &input.logo2x {
        object.key("Logo2x").string_unchecked(&aws_smithy_types::base64::encode(var_2));
    }
    if let Some(var_3) = &input.logo3x {
        object.key("Logo3x").string_unchecked(&aws_smithy_types::base64::encode(var_3));
    }
    if let Some(var_4) = &input.support_email {
        object.key("SupportEmail").string(var_4.as_str());
    }
    if let Some(var_5) = &input.support_link {
        object.key("SupportLink").string(var_5.as_str());
    }
    if let Some(var_6) = &input.forgot_password_link {
        object.key("ForgotPasswordLink").string(var_6.as_str());
    }
    if let Some(var_7) = &input.login_message {
        #[allow(unused_mut)]
        let mut object_8 = object.key("LoginMessage").start_object();
        for (key_9, value_10) in var_7 {
             {
                object_8.key(key_9.as_str()).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    Ok(())
}

