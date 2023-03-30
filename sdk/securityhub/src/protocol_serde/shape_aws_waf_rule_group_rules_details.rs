// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_waf_rule_group_rules_details(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsWafRuleGroupRulesDetails) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.action {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Action").start_object();
        crate::protocol_serde::shape_aws_waf_rule_group_rules_action_details::ser_aws_waf_rule_group_rules_action_details(&mut object_2, var_1)?;
        object_2.finish();
    }
    if input.priority != 0 {
        object.key("Priority").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.priority).into()));
    }
    if let Some(var_3) = &input.rule_id {
        object.key("RuleId").string(var_3.as_str());
    }
    if let Some(var_4) = &input.r#type {
        object.key("Type").string(var_4.as_str());
    }
    Ok(())
}

pub(crate) fn de_aws_waf_rule_group_rules_details<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsWafRuleGroupRulesDetails>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_waf_rule_group_rules_details::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Action" => {
                                builder = builder.set_action(
                                    crate::protocol_serde::shape_aws_waf_rule_group_rules_action_details::de_aws_waf_rule_group_rules_action_details(tokens)?
                                );
                            }
                            "Priority" => {
                                builder = builder.set_priority(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "RuleId" => {
                                builder = builder.set_rule_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Type" => {
                                builder = builder.set_type(
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

