// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_number_filter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::NumberFilter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.upper_inclusive {
        object.key("upperInclusive").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_1).into()));
    }
    if let Some(var_2) = &input.lower_inclusive {
        object.key("lowerInclusive").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_2).into()));
    }
    Ok(())
}

pub(crate) fn de_number_filter<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::NumberFilter>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::number_filter::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "upperInclusive" => {
                                builder = builder.set_upper_inclusive(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy())
                                );
                            }
                            "lowerInclusive" => {
                                builder = builder.set_lower_inclusive(
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

