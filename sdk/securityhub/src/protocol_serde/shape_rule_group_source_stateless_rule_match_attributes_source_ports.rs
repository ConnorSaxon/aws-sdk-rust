// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_rule_group_source_stateless_rule_match_attributes_source_ports(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::RuleGroupSourceStatelessRuleMatchAttributesSourcePorts) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if input.from_port != 0 {
        object.key("FromPort").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.from_port).into()));
    }
    if input.to_port != 0 {
        object.key("ToPort").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((input.to_port).into()));
    }
    Ok(())
}

pub(crate) fn de_rule_group_source_stateless_rule_match_attributes_source_ports<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::RuleGroupSourceStatelessRuleMatchAttributesSourcePorts>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::rule_group_source_stateless_rule_match_attributes_source_ports::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "FromPort" => {
                                builder = builder.set_from_port(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "ToPort" => {
                                builder = builder.set_to_port(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
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

