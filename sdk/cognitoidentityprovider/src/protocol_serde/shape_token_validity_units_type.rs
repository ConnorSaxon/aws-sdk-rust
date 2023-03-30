// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_token_validity_units_type(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::TokenValidityUnitsType) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.access_token {
        object.key("AccessToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.id_token {
        object.key("IdToken").string(var_2.as_str());
    }
    if let Some(var_3) = &input.refresh_token {
        object.key("RefreshToken").string(var_3.as_str());
    }
    Ok(())
}

pub(crate) fn de_token_validity_units_type<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::TokenValidityUnitsType>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::token_validity_units_type::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "AccessToken" => {
                                builder = builder.set_access_token(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::TimeUnitsType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "IdToken" => {
                                builder = builder.set_id_token(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::TimeUnitsType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "RefreshToken" => {
                                builder = builder.set_refresh_token(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::TimeUnitsType::from(u.as_ref())
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

