// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_variable_value<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::VariableValue>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    let mut variant = None;
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => return Ok(None),
                                Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        if variant.is_some() {
                                                            return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("encountered mixed variants in union"));
                                                        }
                        variant = match key.to_unescaped()?.as_ref() {
                            "boolValue" => {
                                Some(crate::model::VariableValue::BoolValue(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                    .unwrap_or_default()
                                ))
                            }
                            "stringValue" => {
                                Some(crate::model::VariableValue::StringValue(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                    .unwrap_or_default()
                                ))
                            }
                            "longValue" => {
                                Some(crate::model::VariableValue::LongValue(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i64::try_from)
                                                        .transpose()?
                                    .unwrap_or_default()
                                ))
                            }
                            "doubleValue" => {
                                Some(crate::model::VariableValue::DoubleValue(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy())
                                    .unwrap_or_default()
                                ))
                            }
                            _ => {
                                                                      aws_smithy_json::deserialize::token::skip_value(tokens)?;
                                                                      Some(crate::model::VariableValue::Unknown)
                                                                    }
                        };
                    }
                    other => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom(format!("expected object key or end object, found: {:?}", other)))
                }
            }
        }
        _ => return Err(aws_smithy_json::deserialize::error::DeserializeError::custom("expected start object or null"))
    }
    Ok(variant)
}

pub fn ser_variable_value(object_3: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::VariableValue) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    match input {
        crate::model::VariableValue::BoolValue(inner) => {
             {
                object_3.key("boolValue").boolean(*inner);
            }
        },
        crate::model::VariableValue::StringValue(inner) => {
             {
                object_3.key("stringValue").string(inner.as_str());
            }
        },
        crate::model::VariableValue::LongValue(inner) => {
             {
                object_3.key("longValue").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*inner).into()));
            }
        },
        crate::model::VariableValue::DoubleValue(inner) => {
             {
                object_3.key("doubleValue").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*inner).into()));
            }
        },
        crate::model::VariableValue::Unknown => return Err(aws_smithy_http::operation::error::SerializationError::unknown_variant("VariableValue"))
    }
    Ok(())
}

