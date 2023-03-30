// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_custom_parameter_values(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::CustomParameterValues) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.string_values {
        let mut array_2 = object.key("StringValues").start_array();
        for item_3 in var_1 {
             {
                array_2.value().string(item_3.as_str());
            }
        }
        array_2.finish();
    }
    if let Some(var_4) = &input.integer_values {
        let mut array_5 = object.key("IntegerValues").start_array();
        for item_6 in var_4 {
             {
                array_5.value().number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::NegInt((*item_6).into()));
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.decimal_values {
        let mut array_8 = object.key("DecimalValues").start_array();
        for item_9 in var_7 {
             {
                array_8.value().number(#[allow(clippy::useless_conversion)]aws_smithy_types::Number::Float((*item_9).into()));
            }
        }
        array_8.finish();
    }
    if let Some(var_10) = &input.date_time_values {
        let mut array_11 = object.key("DateTimeValues").start_array();
        for item_12 in var_10 {
             {
                array_11.value().date_time(item_12, aws_smithy_types::date_time::Format::EpochSeconds)?;
            }
        }
        array_11.finish();
    }
    Ok(())
}

pub(crate) fn de_custom_parameter_values<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::CustomParameterValues>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::custom_parameter_values::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "StringValues" => {
                                builder = builder.set_string_values(
                                    crate::protocol_serde::shape_string_default_value_list::de_string_default_value_list(tokens)?
                                );
                            }
                            "IntegerValues" => {
                                builder = builder.set_integer_values(
                                    crate::protocol_serde::shape_integer_default_value_list::de_integer_default_value_list(tokens)?
                                );
                            }
                            "DecimalValues" => {
                                builder = builder.set_decimal_values(
                                    crate::protocol_serde::shape_decimal_default_value_list::de_decimal_default_value_list(tokens)?
                                );
                            }
                            "DateTimeValues" => {
                                builder = builder.set_date_time_values(
                                    crate::protocol_serde::shape_date_time_default_value_list::de_date_time_default_value_list(tokens)?
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

