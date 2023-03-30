// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_health_check(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::HealthCheck) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.command {
        let mut array_2 = object.key("command").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.interval {
        object.key("interval").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_4).into()));
    }
    if let Some(var_5) = &input.timeout {
        object.key("timeout").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_5).into()));
    }
    if let Some(var_6) = &input.retries {
        object.key("retries").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_6).into()));
    }
    if let Some(var_7) = &input.start_period {
        object.key("startPeriod").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_7).into()));
    }
    Ok(())
}

pub(crate) fn de_health_check<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::HealthCheck>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::health_check::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "command" => {
                                builder = builder.set_command(
                                    crate::protocol_serde::shape_string_list::de_string_list(tokens)?
                                );
                            }
                            "interval" => {
                                builder = builder.set_interval(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "timeout" => {
                                builder = builder.set_timeout(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "retries" => {
                                builder = builder.set_retries(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "startPeriod" => {
                                builder = builder.set_start_period(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
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

