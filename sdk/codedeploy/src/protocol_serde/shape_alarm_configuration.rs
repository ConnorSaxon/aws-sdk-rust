// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_alarm_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AlarmConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.enabled {
        object.key("enabled").boolean(input.enabled);
    }
    if input.ignore_poll_alarm_failure {
        object.key("ignorePollAlarmFailure").boolean(input.ignore_poll_alarm_failure);
    }
    if let Some(var_1) = &input.alarms {
        let mut array_2 = object.key("alarms").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_alarm::ser_alarm(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    Ok(())
}

pub(crate) fn de_alarm_configuration<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AlarmConfiguration>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::alarm_configuration::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "enabled" => {
                                builder = builder.set_enabled(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "ignorePollAlarmFailure" => {
                                builder = builder.set_ignore_poll_alarm_failure(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "alarms" => {
                                builder = builder.set_alarms(
                                    crate::protocol_serde::shape_alarm_list::de_alarm_list(tokens)?
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

