// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_what_if_point_scenario(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::WhatIfPointScenario) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.date {
        object.key("Date").date_time(var_1, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
     {
        object.key("Value").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((input.value).into()));
    }
    Ok(())
}

pub(crate) fn de_what_if_point_scenario<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::WhatIfPointScenario>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::what_if_point_scenario::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Date" => {
                                builder = builder.set_date(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "Value" => {
                                builder = builder.set_value(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy())
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

