// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_block_public_access_configuration<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::BlockPublicAccessConfiguration>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::block_public_access_configuration::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "BlockPublicSecurityGroupRules" => {
                                builder = builder.set_block_public_security_group_rules(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "PermittedPublicSecurityGroupRuleRanges" => {
                                builder = builder.set_permitted_public_security_group_rule_ranges(
                                    crate::protocol_serde::shape_port_ranges::de_port_ranges(tokens)?
                                );
                            }
                            "Classification" => {
                                builder = builder.set_classification(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Configurations" => {
                                builder = builder.set_configurations(
                                    crate::protocol_serde::shape_configuration_list::de_configuration_list(tokens)?
                                );
                            }
                            "Properties" => {
                                builder = builder.set_properties(
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

pub fn ser_block_public_access_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::BlockPublicAccessConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
     {
        object.key("BlockPublicSecurityGroupRules").boolean(input.block_public_security_group_rules);
    }
    if let Some(var_1) = &input.permitted_public_security_group_rule_ranges {
        let mut array_2 = object.key("PermittedPublicSecurityGroupRuleRanges").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_port_range::ser_port_range(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.classification {
        object.key("Classification").string(var_5.as_str());
    }
    if let Some(var_6) = &input.configurations {
        let mut array_7 = object.key("Configurations").start_array();
        for item_8 in var_6 {
             {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_configuration::ser_configuration(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.properties {
        #[allow(unused_mut)]
        let mut object_11 = object.key("Properties").start_object();
        for (key_12, value_13) in var_10 {
             {
                object_11.key(key_12.as_str()).string(value_13.as_str());
            }
        }
        object_11.finish();
    }
    Ok(())
}

