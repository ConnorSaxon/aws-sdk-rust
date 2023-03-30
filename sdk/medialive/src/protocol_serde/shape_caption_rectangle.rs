// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_caption_rectangle(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::CaptionRectangle) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
     {
        object.key("height").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((input.height).into()));
    }
     {
        object.key("leftOffset").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((input.left_offset).into()));
    }
     {
        object.key("topOffset").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((input.top_offset).into()));
    }
     {
        object.key("width").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((input.width).into()));
    }
    Ok(())
}

pub(crate) fn de_caption_rectangle<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::CaptionRectangle>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::caption_rectangle::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "height" => {
                                builder = builder.set_height(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy())
                                );
                            }
                            "leftOffset" => {
                                builder = builder.set_left_offset(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy())
                                );
                            }
                            "topOffset" => {
                                builder = builder.set_top_offset(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy())
                                );
                            }
                            "width" => {
                                builder = builder.set_width(
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

