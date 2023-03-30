// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_waf_regional_rule_details(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsWafRegionalRuleDetails) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.metric_name {
        object.key("MetricName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("Name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.predicate_list {
        let mut array_4 = object.key("PredicateList").start_array();
        for item_5 in var_3 {
             {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_aws_waf_regional_rule_predicate_list_details::ser_aws_waf_regional_rule_predicate_list_details(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.rule_id {
        object.key("RuleId").string(var_7.as_str());
    }
    Ok(())
}

pub(crate) fn de_aws_waf_regional_rule_details<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsWafRegionalRuleDetails>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_waf_regional_rule_details::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "MetricName" => {
                                builder = builder.set_metric_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Name" => {
                                builder = builder.set_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "PredicateList" => {
                                builder = builder.set_predicate_list(
                                    crate::protocol_serde::shape_aws_waf_regional_rule_predicate_list::de_aws_waf_regional_rule_predicate_list(tokens)?
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

