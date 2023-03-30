// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_simple_scope_term(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SimpleScopeTerm) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.comparator {
        object.key("comparator").string(var_1.as_str());
    }
    if let Some(var_2) = &input.key {
        object.key("key").string(var_2.as_str());
    }
    if let Some(var_3) = &input.values {
        let mut array_4 = object.key("values").start_array();
        for item_5 in var_3 {
             {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    Ok(())
}

pub(crate) fn de_simple_scope_term<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::SimpleScopeTerm>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::simple_scope_term::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "comparator" => {
                                builder = builder.set_comparator(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::JobComparator::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "key" => {
                                builder = builder.set_key(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ScopeFilterKey::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "values" => {
                                builder = builder.set_values(
                                    crate::protocol_serde::shape___list_of__string::de___list_of__string(tokens)?
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

