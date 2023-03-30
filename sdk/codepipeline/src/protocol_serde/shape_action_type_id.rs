// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_action_type_id(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::ActionTypeId) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.category {
        object.key("category").string(var_1.as_str());
    }
    if let Some(var_2) = &input.owner {
        object.key("owner").string(var_2.as_str());
    }
    if let Some(var_3) = &input.provider {
        object.key("provider").string(var_3.as_str());
    }
    if let Some(var_4) = &input.version {
        object.key("version").string(var_4.as_str());
    }
    Ok(())
}

pub(crate) fn de_action_type_id<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::ActionTypeId>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::action_type_id::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "category" => {
                                builder = builder.set_category(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ActionCategory::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "owner" => {
                                builder = builder.set_owner(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ActionOwner::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "provider" => {
                                builder = builder.set_provider(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "version" => {
                                builder = builder.set_version(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
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

