// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_schedule_action<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ScheduleAction>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::schedule_action::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "actionName" => {
                                builder = builder.set_action_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "scheduleActionSettings" => {
                                builder = builder.set_schedule_action_settings(
                                    crate::protocol_serde::shape_schedule_action_settings::de_schedule_action_settings(tokens)?
                                );
                            }
                            "scheduleActionStartSettings" => {
                                builder = builder.set_schedule_action_start_settings(
                                    crate::protocol_serde::shape_schedule_action_start_settings::de_schedule_action_start_settings(tokens)?
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

pub fn ser_schedule_action(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ScheduleAction) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.action_name {
        object.key("actionName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.schedule_action_settings {
        #[allow(unused_mut)]
        let mut object_3 = object.key("scheduleActionSettings").start_object();
        crate::protocol_serde::shape_schedule_action_settings::ser_schedule_action_settings(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.schedule_action_start_settings {
        #[allow(unused_mut)]
        let mut object_5 = object.key("scheduleActionStartSettings").start_object();
        crate::protocol_serde::shape_schedule_action_start_settings::ser_schedule_action_start_settings(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}

