// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_custom_filter_configuration(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::CustomFilterConfiguration) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.match_operator {
        object.key("MatchOperator").string(var_1.as_str());
    }
    if let Some(var_2) = &input.category_value {
        object.key("CategoryValue").string(var_2.as_str());
    }
    if let Some(var_3) = &input.select_all_options {
        object.key("SelectAllOptions").string(var_3.as_str());
    }
    if let Some(var_4) = &input.parameter_name {
        object.key("ParameterName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.null_option {
        object.key("NullOption").string(var_5.as_str());
    }
    Ok(())
}

pub(crate) fn de_custom_filter_configuration<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::CustomFilterConfiguration>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::custom_filter_configuration::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "MatchOperator" => {
                                builder = builder.set_match_operator(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::CategoryFilterMatchOperator::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "CategoryValue" => {
                                builder = builder.set_category_value(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "SelectAllOptions" => {
                                builder = builder.set_select_all_options(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::CategoryFilterSelectAllOptions::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "ParameterName" => {
                                builder = builder.set_parameter_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "NullOption" => {
                                builder = builder.set_null_option(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::FilterNullOption::from(u.as_ref())
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

