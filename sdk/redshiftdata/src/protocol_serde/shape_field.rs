// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_field<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::Field>, aws_smithy_json::deserialize::error::DeserializeError>
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
                            "isNull" => {
                                Some(crate::model::Field::IsNull(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                    .unwrap_or_default()
                                ))
                            }
                            "booleanValue" => {
                                Some(crate::model::Field::BooleanValue(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                    .unwrap_or_default()
                                ))
                            }
                            "longValue" => {
                                Some(crate::model::Field::LongValue(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i64::try_from)
                                                        .transpose()?
                                    .unwrap_or_default()
                                ))
                            }
                            "doubleValue" => {
                                Some(crate::model::Field::DoubleValue(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy())
                                    .unwrap_or_default()
                                ))
                            }
                            "stringValue" => {
                                Some(crate::model::Field::StringValue(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                    .unwrap_or_default()
                                ))
                            }
                            "blobValue" => {
                                Some(crate::model::Field::BlobValue(
                                    aws_smithy_json::deserialize::token::expect_blob_or_null(tokens.next())?
                                    .ok_or_else(|| aws_smithy_json::deserialize::error::DeserializeError::custom("value for 'blobValue' cannot be null"))?
                                ))
                            }
                            _ => {
                                                                      aws_smithy_json::deserialize::token::skip_value(tokens)?;
                                                                      Some(crate::model::Field::Unknown)
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

