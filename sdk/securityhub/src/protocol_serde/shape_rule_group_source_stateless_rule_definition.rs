// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_rule_group_source_stateless_rule_definition(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RuleGroupSourceStatelessRuleDefinition) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.actions {
        let mut array_2 = object.key("Actions").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.match_attributes {
        #[allow(unused_mut)]
        let mut object_5 = object.key("MatchAttributes").start_object();
        crate::protocol_serde::shape_rule_group_source_stateless_rule_match_attributes::ser_rule_group_source_stateless_rule_match_attributes(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}

pub(crate) fn de_rule_group_source_stateless_rule_definition<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::RuleGroupSourceStatelessRuleDefinition>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::rule_group_source_stateless_rule_definition::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Actions" => {
                                builder = builder.set_actions(
                                    crate::protocol_serde::shape_non_empty_string_list::de_non_empty_string_list(tokens)?
                                );
                            }
                            "MatchAttributes" => {
                                builder = builder.set_match_attributes(
                                    crate::protocol_serde::shape_rule_group_source_stateless_rule_match_attributes::de_rule_group_source_stateless_rule_match_attributes(tokens)?
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

