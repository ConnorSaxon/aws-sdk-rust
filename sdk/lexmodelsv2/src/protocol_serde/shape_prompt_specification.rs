// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_prompt_specification<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::PromptSpecification>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::prompt_specification::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "messageGroups" => {
                                builder = builder.set_message_groups(
                                    crate::protocol_serde::shape_message_groups_list::de_message_groups_list(tokens)?
                                );
                            }
                            "maxRetries" => {
                                builder = builder.set_max_retries(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "allowInterrupt" => {
                                builder = builder.set_allow_interrupt(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "messageSelectionStrategy" => {
                                builder = builder.set_message_selection_strategy(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::MessageSelectionStrategy::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "promptAttemptsSpecification" => {
                                builder = builder.set_prompt_attempts_specification(
                                    crate::protocol_serde::shape_prompt_attempts_specification_map::de_prompt_attempts_specification_map(tokens)?
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

pub fn ser_prompt_specification(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::PromptSpecification) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.message_groups {
        let mut array_2 = object.key("messageGroups").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_message_group::ser_message_group(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.max_retries {
        object.key("maxRetries").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_5).into()));
    }
    if let Some(var_6) = &input.allow_interrupt {
        object.key("allowInterrupt").boolean(*var_6);
    }
    if let Some(var_7) = &input.message_selection_strategy {
        object.key("messageSelectionStrategy").string(var_7.as_str());
    }
    if let Some(var_8) = &input.prompt_attempts_specification {
        #[allow(unused_mut)]
        let mut object_9 = object.key("promptAttemptsSpecification").start_object();
        for (key_10, value_11) in var_8 {
             {
                #[allow(unused_mut)]
                let mut object_12 = object_9.key(key_10.as_str()).start_object();
                crate::protocol_serde::shape_prompt_attempt_specification::ser_prompt_attempt_specification(&mut object_12, value_11)?;
                object_12.finish();
            }
        }
        object_9.finish();
    }
    Ok(())
}

