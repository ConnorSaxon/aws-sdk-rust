// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_slot(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::Slot) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.description {
        object.key("description").string(var_2.as_str());
    }
    if let Some(var_3) = &input.slot_constraint {
        object.key("slotConstraint").string(var_3.as_str());
    }
    if let Some(var_4) = &input.slot_type {
        object.key("slotType").string(var_4.as_str());
    }
    if let Some(var_5) = &input.slot_type_version {
        object.key("slotTypeVersion").string(var_5.as_str());
    }
    if let Some(var_6) = &input.value_elicitation_prompt {
        #[allow(unused_mut)]
        let mut object_7 = object.key("valueElicitationPrompt").start_object();
        crate::protocol_serde::shape_prompt::ser_prompt(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.priority {
        object.key("priority").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_8).into()));
    }
    if let Some(var_9) = &input.sample_utterances {
        let mut array_10 = object.key("sampleUtterances").start_array();
        for item_11 in var_9 {
             {
                array_10.value().string(item_11.as_str());
            }
        }
        array_10.finish();
    }
    if let Some(var_12) = &input.response_card {
        object.key("responseCard").string(var_12.as_str());
    }
    if let Some(var_13) = &input.obfuscation_setting {
        object.key("obfuscationSetting").string(var_13.as_str());
    }
    if let Some(var_14) = &input.default_value_spec {
        #[allow(unused_mut)]
        let mut object_15 = object.key("defaultValueSpec").start_object();
        crate::protocol_serde::shape_slot_default_value_spec::ser_slot_default_value_spec(&mut object_15, var_14)?;
        object_15.finish();
    }
    Ok(())
}

pub(crate) fn de_slot<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::Slot>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::slot::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "name" => {
                                builder = builder.set_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "description" => {
                                builder = builder.set_description(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "slotConstraint" => {
                                builder = builder.set_slot_constraint(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::SlotConstraint::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "slotType" => {
                                builder = builder.set_slot_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "slotTypeVersion" => {
                                builder = builder.set_slot_type_version(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "valueElicitationPrompt" => {
                                builder = builder.set_value_elicitation_prompt(
                                    crate::protocol_serde::shape_prompt::de_prompt(tokens)?
                                );
                            }
                            "priority" => {
                                builder = builder.set_priority(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "sampleUtterances" => {
                                builder = builder.set_sample_utterances(
                                    crate::protocol_serde::shape_slot_utterance_list::de_slot_utterance_list(tokens)?
                                );
                            }
                            "responseCard" => {
                                builder = builder.set_response_card(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "obfuscationSetting" => {
                                builder = builder.set_obfuscation_setting(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ObfuscationSetting::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "defaultValueSpec" => {
                                builder = builder.set_default_value_spec(
                                    crate::protocol_serde::shape_slot_default_value_spec::de_slot_default_value_spec(tokens)?
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

