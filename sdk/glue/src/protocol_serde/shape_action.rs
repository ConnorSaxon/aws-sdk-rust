// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_action(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Action) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.job_name {
        object.key("JobName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.arguments {
        #[allow(unused_mut)]
        let mut object_3 = object.key("Arguments").start_object();
        for (key_4, value_5) in var_2 {
             {
                object_3.key(key_4.as_str()).string(value_5.as_str());
            }
        }
        object_3.finish();
    }
    if let Some(var_6) = &input.timeout {
        object.key("Timeout").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_6).into()));
    }
    if let Some(var_7) = &input.security_configuration {
        object.key("SecurityConfiguration").string(var_7.as_str());
    }
    if let Some(var_8) = &input.notification_property {
        #[allow(unused_mut)]
        let mut object_9 = object.key("NotificationProperty").start_object();
        crate::protocol_serde::shape_notification_property::ser_notification_property(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.crawler_name {
        object.key("CrawlerName").string(var_10.as_str());
    }
    Ok(())
}

pub(crate) fn de_action<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::Action>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::action::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "JobName" => {
                                builder = builder.set_job_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Arguments" => {
                                builder = builder.set_arguments(
                                    crate::protocol_serde::shape_generic_map::de_generic_map(tokens)?
                                );
                            }
                            "Timeout" => {
                                builder = builder.set_timeout(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "SecurityConfiguration" => {
                                builder = builder.set_security_configuration(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "NotificationProperty" => {
                                builder = builder.set_notification_property(
                                    crate::protocol_serde::shape_notification_property::de_notification_property(tokens)?
                                );
                            }
                            "CrawlerName" => {
                                builder = builder.set_crawler_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
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

