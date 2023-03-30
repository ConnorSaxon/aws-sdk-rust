// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_audit_notification_target(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AuditNotificationTarget) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.target_arn {
        object.key("targetArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.role_arn {
        object.key("roleArn").string(var_2.as_str());
    }
    if input.enabled {
        object.key("enabled").boolean(input.enabled);
    }
    Ok(())
}

pub(crate) fn de_audit_notification_target<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AuditNotificationTarget>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::audit_notification_target::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "targetArn" => {
                                builder = builder.set_target_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "roleArn" => {
                                builder = builder.set_role_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "enabled" => {
                                builder = builder.set_enabled(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
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

