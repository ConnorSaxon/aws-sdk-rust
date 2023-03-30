// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_skewed_info<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::SkewedInfo>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::skewed_info::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "SkewedColumnNames" => {
                                builder = builder.set_skewed_column_names(
                                    crate::protocol_serde::shape_name_string_list::de_name_string_list(tokens)?
                                );
                            }
                            "SkewedColumnValues" => {
                                builder = builder.set_skewed_column_values(
                                    crate::protocol_serde::shape_column_value_string_list::de_column_value_string_list(tokens)?
                                );
                            }
                            "SkewedColumnValueLocationMaps" => {
                                builder = builder.set_skewed_column_value_location_maps(
                                    crate::protocol_serde::shape_location_map::de_location_map(tokens)?
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

pub fn ser_skewed_info(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::SkewedInfo) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.skewed_column_names {
        let mut array_2 = object.key("SkewedColumnNames").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.skewed_column_values {
        let mut array_5 = object.key("SkewedColumnValues").start_array();
        for item_6 in var_4 {
             {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.skewed_column_value_location_maps {
        #[allow(unused_mut)]
        let mut object_8 = object.key("SkewedColumnValueLocationMaps").start_object();
        for (key_9, value_10) in var_7 {
             {
                object_8.key(key_9.as_str()).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    Ok(())
}

