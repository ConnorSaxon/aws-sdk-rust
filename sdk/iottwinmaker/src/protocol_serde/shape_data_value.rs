// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_data_value(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DataValue) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.boolean_value {
        object.key("booleanValue").boolean(*var_1);
    }
    if let Some(var_2) = &input.double_value {
        object.key("doubleValue").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*var_2).into()));
    }
    if let Some(var_3) = &input.integer_value {
        object.key("integerValue").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_3).into()));
    }
    if let Some(var_4) = &input.long_value {
        object.key("longValue").number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*var_4).into()));
    }
    if let Some(var_5) = &input.string_value {
        object.key("stringValue").string(var_5.as_str());
    }
    if let Some(var_6) = &input.list_value {
        let mut array_7 = object.key("listValue").start_array();
        for item_8 in var_6 {
             {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_data_value::ser_data_value(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.map_value {
        #[allow(unused_mut)]
        let mut object_11 = object.key("mapValue").start_object();
        for (key_12, value_13) in var_10 {
             {
                #[allow(unused_mut)]
                let mut object_14 = object_11.key(key_12.as_str()).start_object();
                crate::protocol_serde::shape_data_value::ser_data_value(&mut object_14, value_13)?;
                object_14.finish();
            }
        }
        object_11.finish();
    }
    if let Some(var_15) = &input.relationship_value {
        #[allow(unused_mut)]
        let mut object_16 = object.key("relationshipValue").start_object();
        crate::protocol_serde::shape_relationship_value::ser_relationship_value(&mut object_16, var_15)?;
        object_16.finish();
    }
    if let Some(var_17) = &input.expression {
        object.key("expression").string(var_17.as_str());
    }
    Ok(())
}

pub(crate) fn de_data_value<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::DataValue>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::data_value::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "booleanValue" => {
                                builder = builder.set_boolean_value(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "doubleValue" => {
                                builder = builder.set_double_value(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f64_lossy())
                                );
                            }
                            "integerValue" => {
                                builder = builder.set_integer_value(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i32::try_from)
                                                        .transpose()?
                                );
                            }
                            "longValue" => {
                                builder = builder.set_long_value(
                                    aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                                        .map(i64::try_from)
                                                        .transpose()?
                                );
                            }
                            "stringValue" => {
                                builder = builder.set_string_value(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "listValue" => {
                                builder = builder.set_list_value(
                                    crate::protocol_serde::shape_data_value_list::de_data_value_list(tokens)?
                                );
                            }
                            "mapValue" => {
                                builder = builder.set_map_value(
                                    crate::protocol_serde::shape_data_value_map::de_data_value_map(tokens)?
                                );
                            }
                            "relationshipValue" => {
                                builder = builder.set_relationship_value(
                                    crate::protocol_serde::shape_relationship_value::de_relationship_value(tokens)?
                                );
                            }
                            "expression" => {
                                builder = builder.set_expression(
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

