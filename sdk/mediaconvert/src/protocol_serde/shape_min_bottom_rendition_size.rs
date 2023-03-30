// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_min_bottom_rendition_size(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::MinBottomRenditionSize) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.height != 0 {
        object.key("height").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.height).into()));
    }
    if input.width != 0 {
        object.key("width").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.width).into()));
    }
    Ok(())
}

pub(crate) fn de_min_bottom_rendition_size<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::MinBottomRenditionSize>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::min_bottom_rendition_size::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "height" => {
                                builder = builder.set_height(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "width" => {
                                builder = builder.set_width(
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

