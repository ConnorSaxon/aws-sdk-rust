// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_dialog_state<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::DialogState>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::dialog_state::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "dialogAction" => {
                                builder = builder.set_dialog_action(
                                    crate::protocol_serde::shape_dialog_action::de_dialog_action(tokens)?
                                );
                            }
                            "intent" => {
                                builder = builder.set_intent(
                                    crate::protocol_serde::shape_intent_override::de_intent_override(tokens)?
                                );
                            }
                            "sessionAttributes" => {
                                builder = builder.set_session_attributes(
                                    crate::protocol_serde::shape_string_map::de_string_map(tokens)?
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

pub fn ser_dialog_state(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DialogState) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.dialog_action {
        #[allow(unused_mut)]
        let mut object_2 = object.key("dialogAction").start_object();
        crate::protocol_serde::shape_dialog_action::ser_dialog_action(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.intent {
        #[allow(unused_mut)]
        let mut object_4 = object.key("intent").start_object();
        crate::protocol_serde::shape_intent_override::ser_intent_override(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.session_attributes {
        #[allow(unused_mut)]
        let mut object_6 = object.key("sessionAttributes").start_object();
        for (key_7, value_8) in var_5 {
             {
                object_6.key(key_7.as_str()).string(value_8.as_str());
            }
        }
        object_6.finish();
    }
    Ok(())
}

