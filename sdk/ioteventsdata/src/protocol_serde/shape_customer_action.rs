// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_customer_action<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::CustomerAction>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::customer_action::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "actionName" => {
                                builder = builder.set_action_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::CustomerActionName::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "snoozeActionConfiguration" => {
                                builder = builder.set_snooze_action_configuration(
                                    crate::protocol_serde::shape_snooze_action_configuration::de_snooze_action_configuration(tokens)?
                                );
                            }
                            "enableActionConfiguration" => {
                                builder = builder.set_enable_action_configuration(
                                    crate::protocol_serde::shape_enable_action_configuration::de_enable_action_configuration(tokens)?
                                );
                            }
                            "disableActionConfiguration" => {
                                builder = builder.set_disable_action_configuration(
                                    crate::protocol_serde::shape_disable_action_configuration::de_disable_action_configuration(tokens)?
                                );
                            }
                            "acknowledgeActionConfiguration" => {
                                builder = builder.set_acknowledge_action_configuration(
                                    crate::protocol_serde::shape_acknowledge_action_configuration::de_acknowledge_action_configuration(tokens)?
                                );
                            }
                            "resetActionConfiguration" => {
                                builder = builder.set_reset_action_configuration(
                                    crate::protocol_serde::shape_reset_action_configuration::de_reset_action_configuration(tokens)?
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

