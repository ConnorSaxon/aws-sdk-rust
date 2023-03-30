// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_rate_increase_criteria(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RateIncreaseCriteria) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.number_of_notified_things {
        object.key("numberOfNotifiedThings").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_1).into()));
    }
    if let Some(var_2) = &input.number_of_succeeded_things {
        object.key("numberOfSucceededThings").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_2).into()));
    }
    Ok(())
}

pub(crate) fn de_rate_increase_criteria<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::RateIncreaseCriteria>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::rate_increase_criteria::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "numberOfNotifiedThings" => {
                                builder = builder.set_number_of_notified_things(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "numberOfSucceededThings" => {
                                builder = builder.set_number_of_succeeded_things(
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

