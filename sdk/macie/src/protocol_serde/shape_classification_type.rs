// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_classification_type(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ClassificationType) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.one_time {
        object.key("oneTime").string(var_1.as_str());
    }
    if let Some(var_2) = &input.continuous {
        object.key("continuous").string(var_2.as_str());
    }
    Ok(())
}

pub(crate) fn de_classification_type<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ClassificationType>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::classification_type::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "oneTime" => {
                                builder = builder.set_one_time(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::S3OneTimeClassificationType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "continuous" => {
                                builder = builder.set_continuous(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::S3ContinuousClassificationType::from(u.as_ref())
                                        )
                                    ).transpose()?
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

