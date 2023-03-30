// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_lf_tag_key_resource(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::LfTagKeyResource) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.catalog_id {
        object.key("CatalogId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.tag_key {
        object.key("TagKey").string(var_2.as_str());
    }
    if let Some(var_3) = &input.tag_values {
        let mut array_4 = object.key("TagValues").start_array();
        for item_5 in var_3 {
             {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    Ok(())
}

pub(crate) fn de_lf_tag_key_resource<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::LfTagKeyResource>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::lf_tag_key_resource::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "CatalogId" => {
                                builder = builder.set_catalog_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "TagKey" => {
                                builder = builder.set_tag_key(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "TagValues" => {
                                builder = builder.set_tag_values(
                                    crate::protocol_serde::shape_tag_value_list::de_tag_value_list(tokens)?
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

