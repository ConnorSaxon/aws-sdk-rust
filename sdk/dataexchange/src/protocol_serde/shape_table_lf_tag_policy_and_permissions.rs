// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_table_lf_tag_policy_and_permissions(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::TableLfTagPolicyAndPermissions) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.expression {
        let mut array_2 = object.key("Expression").start_array();
        for item_3 in var_1 {
             {
                #[allow(unused_mut)]
                let mut object_4 = array_2.value().start_object();
                crate::protocol_serde::shape_lf_tag::ser_lf_tag(&mut object_4, item_3)?;
                object_4.finish();
            }
        }
        array_2.finish();
    }
    if let Some(var_5) = &input.permissions {
        let mut array_6 = object.key("Permissions").start_array();
        for item_7 in var_5 {
             {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    Ok(())
}

pub(crate) fn de_table_lf_tag_policy_and_permissions<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::TableLfTagPolicyAndPermissions>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::table_lf_tag_policy_and_permissions::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Expression" => {
                                builder = builder.set_expression(
                                    crate::protocol_serde::shape_list_of_lf_tags::de_list_of_lf_tags(tokens)?
                                );
                            }
                            "Permissions" => {
                                builder = builder.set_permissions(
                                    crate::protocol_serde::shape_list_of_table_tag_policy_lf_permissions::de_list_of_table_tag_policy_lf_permissions(tokens)?
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

