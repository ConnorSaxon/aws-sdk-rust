// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_organization_managed_rule_metadata(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::OrganizationManagedRuleMetadata) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("Description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.rule_identifier {
        object.key("RuleIdentifier").string(var_2.as_str());
    }
    if let Some(var_3) = &input.input_parameters {
        object.key("InputParameters").string(var_3.as_str());
    }
    if let Some(var_4) = &input.maximum_execution_frequency {
        object.key("MaximumExecutionFrequency").string(var_4.as_str());
    }
    if let Some(var_5) = &input.resource_types_scope {
        let mut array_6 = object.key("ResourceTypesScope").start_array();
        for item_7 in var_5 {
             {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.resource_id_scope {
        object.key("ResourceIdScope").string(var_8.as_str());
    }
    if let Some(var_9) = &input.tag_key_scope {
        object.key("TagKeyScope").string(var_9.as_str());
    }
    if let Some(var_10) = &input.tag_value_scope {
        object.key("TagValueScope").string(var_10.as_str());
    }
    Ok(())
}

pub(crate) fn de_organization_managed_rule_metadata<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::OrganizationManagedRuleMetadata>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::organization_managed_rule_metadata::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Description" => {
                                builder = builder.set_description(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "RuleIdentifier" => {
                                builder = builder.set_rule_identifier(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "InputParameters" => {
                                builder = builder.set_input_parameters(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "MaximumExecutionFrequency" => {
                                builder = builder.set_maximum_execution_frequency(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::MaximumExecutionFrequency::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "ResourceTypesScope" => {
                                builder = builder.set_resource_types_scope(
                                    crate::protocol_serde::shape_resource_types_scope::de_resource_types_scope(tokens)?
                                );
                            }
                            "ResourceIdScope" => {
                                builder = builder.set_resource_id_scope(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "TagKeyScope" => {
                                builder = builder.set_tag_key_scope(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "TagValueScope" => {
                                builder = builder.set_tag_value_scope(
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

