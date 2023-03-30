// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_data_field_series_item(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::DataFieldSeriesItem) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.field_id {
        object.key("FieldId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.field_value {
        object.key("FieldValue").string(var_2.as_str());
    }
    if let Some(var_3) = &input.axis_binding {
        object.key("AxisBinding").string(var_3.as_str());
    }
    if let Some(var_4) = &input.settings {
        #[allow(unused_mut)]
        let mut object_5 = object.key("Settings").start_object();
        crate::protocol_serde::shape_line_chart_series_settings::ser_line_chart_series_settings(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}

pub(crate) fn de_data_field_series_item<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::DataFieldSeriesItem>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::data_field_series_item::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "FieldId" => {
                                builder = builder.set_field_id(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "FieldValue" => {
                                builder = builder.set_field_value(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            u.into_owned()
                                        )
                                    ).transpose()?
                                );
                            }
                            "AxisBinding" => {
                                builder = builder.set_axis_binding(
                                    aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?.map(|s|
                                        s.to_unescaped().map(|u|
                                            crate::model::AxisBinding::from(u.as_ref())
                                        )
                                    ).transpose()?
                                );
                            }
                            "Settings" => {
                                builder = builder.set_settings(
                                    crate::protocol_serde::shape_line_chart_series_settings::de_line_chart_series_settings(tokens)?
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

