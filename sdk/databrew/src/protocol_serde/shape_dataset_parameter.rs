// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_dataset_parameter(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DatasetParameter) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.r#type {
        object.key("Type").string(var_2.as_str());
    }
    if let Some(var_3) = &input.datetime_options {
        #[allow(unused_mut)]
        let mut object_4 = object.key("DatetimeOptions").start_object();
        crate::protocol_serde::shape_datetime_options::ser_datetime_options(&mut object_4, var_3)?;
        object_4.finish();
    }
    if input.create_column {
        object.key("CreateColumn").boolean(input.create_column);
    }
    if let Some(var_5) = &input.filter {
        #[allow(unused_mut)]
        let mut object_6 = object.key("Filter").start_object();
        crate::protocol_serde::shape_filter_expression::ser_filter_expression(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}

pub(crate) fn de_dataset_parameter<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::DatasetParameter>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::dataset_parameter::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "Name" => {
                                builder = builder.set_name(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "Type" => {
                                builder = builder.set_type(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::ParameterType::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "DatetimeOptions" => {
                                builder = builder.set_datetime_options(
                                    crate::protocol_serde::shape_datetime_options::de_datetime_options(tokens)?
                                );
                            }
                            "CreateColumn" => {
                                builder = builder.set_create_column(
                                    aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?
                                );
                            }
                            "Filter" => {
                                builder = builder.set_filter(
                                    crate::protocol_serde::shape_filter_expression::de_filter_expression(tokens)?
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

