// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_bot_alias_locale_settings(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::BotAliasLocaleSettings) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
     {
        object.key("enabled").boolean(input.enabled);
    }
    if let Some(var_1) = &input.code_hook_specification {
        #[allow(unused_mut)]
        let mut object_2 = object.key("codeHookSpecification").start_object();
        crate::protocol_serde::shape_code_hook_specification::ser_code_hook_specification(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

pub(crate) fn de_bot_alias_locale_settings<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::BotAliasLocaleSettings>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::bot_alias_locale_settings::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "enabled" => {
                                builder = builder.set_enabled(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "codeHookSpecification" => {
                                builder = builder.set_code_hook_specification(
                                    crate::protocol_serde::shape_code_hook_specification::de_code_hook_specification(tokens)?
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

