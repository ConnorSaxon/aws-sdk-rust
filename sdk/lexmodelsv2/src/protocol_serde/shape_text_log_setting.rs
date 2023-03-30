// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_text_log_setting(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::TextLogSetting) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
     {
        object.key("enabled").boolean(input.enabled);
    }
    if let Some(var_1) = &input.destination {
        #[allow(unused_mut)]
        let mut object_2 = object.key("destination").start_object();
        crate::protocol_serde::shape_text_log_destination::ser_text_log_destination(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

pub(crate) fn de_text_log_setting<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::TextLogSetting>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::text_log_setting::Builder::default();
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
                            "destination" => {
                                builder = builder.set_destination(
                                    crate::protocol_serde::shape_text_log_destination::de_text_log_destination(tokens)?
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

