// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_location<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::Location>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::location::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "latitude" => {
                                builder = builder.set_latitude(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy())
                                );
                            }
                            "longitude" => {
                                builder = builder.set_longitude(
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

pub fn ser_location(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Location) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.latitude {
        object.key("latitude").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_1).into()));
    }
    if let Some(var_2) = &input.longitude {
        object.key("longitude").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_2).into()));
    }
    Ok(())
}

