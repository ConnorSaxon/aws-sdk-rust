// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_access_rules(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AccessRules) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.get_object {
        object.key("getObject").string(var_1.as_str());
    }
    if let Some(var_2) = &input.allow_public_overrides {
        object.key("allowPublicOverrides").boolean(*var_2);
    }
    Ok(())
}

pub(crate) fn de_access_rules<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AccessRules>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::access_rules::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "getObject" => {
                                builder = builder.set_get_object(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::AccessType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "allowPublicOverrides" => {
                                builder = builder.set_allow_public_overrides(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
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

