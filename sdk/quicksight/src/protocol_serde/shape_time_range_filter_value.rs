// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_time_range_filter_value(object: &mut aws_smithy_json::serialize::JsonObjectWriter, input: &crate::model::TimeRangeFilterValue) -> Result<(), aws_smithy_http::operation::error::SerializationError> {
    if let Some(var_1) = &input.static_value {
        object.key("StaticValue").date_time(var_1, aws_smithy_types::date_time::Format::EpochSeconds)?;
    }
    if let Some(var_2) = &input.rolling_date {
        #[allow(unused_mut)]
        let mut object_3 = object.key("RollingDate").start_object();
        crate::protocol_serde::shape_rolling_date_configuration::ser_rolling_date_configuration(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.parameter {
        object.key("Parameter").string(var_4.as_str());
    }
    Ok(())
}

pub(crate) fn de_time_range_filter_value<'a, I>(tokens: &mut std::iter::Peekable<I>) -> Result<Option<crate::model::TimeRangeFilterValue>, aws_smithy_json::deserialize::error::DeserializeError>
                    where I: Iterator<Item = Result<aws_smithy_json::deserialize::Token<'a>, aws_smithy_json::deserialize::error::DeserializeError>> {
    match tokens.next().transpose()? {
        Some(aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
                        Some(aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::model::time_range_filter_value::Builder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                                            Some(aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => {
                        match key.to_unescaped()?.as_ref() {
                            "StaticValue" => {
                                builder = builder.set_static_value(
                                    aws_smithy_json::deserialize::token::expect_timestamp_or_null(tokens.next(), aws_smithy_types::date_time::Format::EpochSeconds)?
                                );
                            }
                            "RollingDate" => {
                                builder = builder.set_rolling_date(
                                    crate::protocol_serde::shape_rolling_date_configuration::de_rolling_date_configuration(tokens)?
                                );
                            }
                            "Parameter" => {
                                builder = builder.set_parameter(
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

