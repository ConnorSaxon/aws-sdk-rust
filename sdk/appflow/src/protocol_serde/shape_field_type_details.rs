// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_field_type_details<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::FieldTypeDetails>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::field_type_details::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "fieldType" => {
                                builder = builder.set_field_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "filterOperators" => {
                                builder = builder.set_filter_operators(
                                    crate::protocol_serde::shape_filter_operator_list::de_filter_operator_list(tokens)?
                                );
                            }
                            "supportedValues" => {
                                builder = builder.set_supported_values(
                                    crate::protocol_serde::shape_supported_value_list::de_supported_value_list(tokens)?
                                );
                            }
                            "valueRegexPattern" => {
                                builder = builder.set_value_regex_pattern(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "supportedDateFormat" => {
                                builder = builder.set_supported_date_format(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "fieldValueRange" => {
                                builder = builder.set_field_value_range(
                                    crate::protocol_serde::shape_range::de_range(tokens)?
                                );
                            }
                            "fieldLengthRange" => {
                                builder = builder.set_field_length_range(
                                    crate::protocol_serde::shape_range::de_range(tokens)?
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

