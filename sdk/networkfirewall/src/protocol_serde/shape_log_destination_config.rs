// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_log_destination_config(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::LogDestinationConfig) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.log_type {
        object.key("LogType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.log_destination_type {
        object.key("LogDestinationType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.log_destination {
        #[allow(unused_mut)]
        let mut object_4 = object.key("LogDestination").start_object();
        for (key_5, value_6) in var_3 {
             {
                object_4.key(key_5.as_str()).string(value_6.as_str());
            }
        }
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_log_destination_config<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::LogDestinationConfig>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::log_destination_config::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "LogType" => {
                                builder = builder.set_log_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::LogType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "LogDestinationType" => {
                                builder = builder.set_log_destination_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::LogDestinationType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "LogDestination" => {
                                builder = builder.set_log_destination(
                                    crate::protocol_serde::shape_log_destination_map::de_log_destination_map(tokens)?
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

