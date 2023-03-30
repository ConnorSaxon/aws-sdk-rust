// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_alarm_rule(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AlarmRule) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.simple_rule {
        #[allow(unused_mut)]
        let mut object_2 = object.key("simpleRule").start_object();
        crate::protocol_serde::shape_simple_rule::ser_simple_rule(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}

pub(crate) fn de_alarm_rule<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AlarmRule>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::alarm_rule::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "simpleRule" => {
                                builder = builder.set_simple_rule(
                                    crate::protocol_serde::shape_simple_rule::de_simple_rule(tokens)?
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

