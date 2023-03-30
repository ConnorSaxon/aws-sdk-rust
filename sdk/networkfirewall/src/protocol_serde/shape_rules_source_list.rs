// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_rules_source_list(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RulesSourceList) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.targets {
        let mut array_2 = object.key("Targets").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.target_types {
        let mut array_5 = object.key("TargetTypes").start_array();
        for item_6 in var_4 {
             {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.generated_rules_type {
        object.key("GeneratedRulesType").string(var_7.as_str());
    }
    Ok(())
}

pub(crate) fn de_rules_source_list<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::RulesSourceList>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::rules_source_list::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Targets" => {
                                builder = builder.set_targets(
                                    crate::protocol_serde::shape_rule_targets::de_rule_targets(tokens)?
                                );
                            }
                            "TargetTypes" => {
                                builder = builder.set_target_types(
                                    crate::protocol_serde::shape_target_types::de_target_types(tokens)?
                                );
                            }
                            "GeneratedRulesType" => {
                                builder = builder.set_generated_rules_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::GeneratedRulesType::from(u.as_ref())
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

