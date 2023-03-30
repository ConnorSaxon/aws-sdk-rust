// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_backend_auth_verification_message_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::CreateBackendAuthVerificationMessageConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.delivery_method {
        object.key("deliveryMethod").string(var_1.as_str());
    }
    if let Some(var_2) = &input.email_settings {
        #[allow(unused_mut)]
        let mut object_3 = object.key("emailSettings").start_object();
        crate::protocol_serde::shape_email_settings::ser_email_settings(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.sms_settings {
        #[allow(unused_mut)]
        let mut object_5 = object.key("smsSettings").start_object();
        crate::protocol_serde::shape_sms_settings::ser_sms_settings(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}

pub(crate) fn de_create_backend_auth_verification_message_config<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::CreateBackendAuthVerificationMessageConfig>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::create_backend_auth_verification_message_config::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "deliveryMethod" => {
                                builder = builder.set_delivery_method(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::DeliveryMethod::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "emailSettings" => {
                                builder = builder.set_email_settings(
                                    crate::protocol_serde::shape_email_settings::de_email_settings(tokens)?
                                );
                            }
                            "smsSettings" => {
                                builder = builder.set_sms_settings(
                                    crate::protocol_serde::shape_sms_settings::de_sms_settings(tokens)?
                                );
                            }
                            _ => aws_smithy_json::deserialize::token::skip_value(tokens)?
                        }
                    }
                    other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
            Ok(Some(builder.build()))
        }
        _ => {
            Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
        }
    }
}

