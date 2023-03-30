// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_network_firewall_rule_group_details(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::AwsNetworkFirewallRuleGroupDetails) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.capacity != 0 {
        object.key("Capacity").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.capacity).into()));
    }
    if let Some(var_1) = &input.description {
        object.key("Description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.rule_group {
        #[allow(unused_mut)]
        let mut object_3 = object.key("RuleGroup").start_object();
        crate::protocol_serde::shape_rule_group_details::ser_rule_group_details(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.rule_group_arn {
        object.key("RuleGroupArn").string(var_4.as_str());
    }
    if let Some(var_5) = &input.rule_group_id {
        object.key("RuleGroupId").string(var_5.as_str());
    }
    if let Some(var_6) = &input.rule_group_name {
        object.key("RuleGroupName").string(var_6.as_str());
    }
    if let Some(var_7) = &input.r#type {
        object.key("Type").string(var_7.as_str());
    }
    Ok(())
}

pub(crate) fn de_aws_network_firewall_rule_group_details<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::AwsNetworkFirewallRuleGroupDetails>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::aws_network_firewall_rule_group_details::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Capacity" => {
                                builder = builder.set_capacity(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "Description" => {
                                builder = builder.set_description(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "RuleGroup" => {
                                builder = builder.set_rule_group(
                                    crate::protocol_serde::shape_rule_group_details::de_rule_group_details(tokens)?
                                );
                            }
                            "RuleGroupArn" => {
                                builder = builder.set_rule_group_arn(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "RuleGroupId" => {
                                builder = builder.set_rule_group_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "RuleGroupName" => {
                                builder = builder.set_rule_group_name(
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

