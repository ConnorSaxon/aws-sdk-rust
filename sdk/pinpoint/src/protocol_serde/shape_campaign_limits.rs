// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_campaign_limits(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::CampaignLimits) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.daily != 0 {
        object.key("Daily").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.daily).into()));
    }
    if input.maximum_duration != 0 {
        object.key("MaximumDuration").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.maximum_duration).into()));
    }
    if input.messages_per_second != 0 {
        object.key("MessagesPerSecond").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.messages_per_second).into()));
    }
    if input.total != 0 {
        object.key("Total").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.total).into()));
    }
    if input.session != 0 {
        object.key("Session").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.session).into()));
    }
    Ok(())
}

pub(crate) fn de_campaign_limits<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::CampaignLimits>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::campaign_limits::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Daily" => {
                                builder = builder.set_daily(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "MaximumDuration" => {
                                builder = builder.set_maximum_duration(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "MessagesPerSecond" => {
                                builder = builder.set_messages_per_second(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "Total" => {
                                builder = builder.set_total(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "Session" => {
                                builder = builder.set_session(
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

